<#
Starts the full benchmark on a new Hetzner Cloud server that uploads its files to a Storage Box
and deletes itself at the end (finish-hetzner.sh), or at a deadline if anything hangs. Nothing
needs to watch it: afterwards the files are in the Storage Box folder named by -RunName.

    .\launch-hetzner.ps1 -TokenFile X:\keys\hetzner.md -StorageFile X:\keys\storage.md
    .\launch-hetzner.ps1 ... -Type cx23 -DeadlineHours 0.25 -SelfTest   # a few cents' check first

-TokenFile holds "MAIN TOKEN: <64 chars>" (used here, on this PC) and "BENCH TOKEN: <64 chars>"
(copied to the server, root only, so it can delete itself). -StorageFile holds "HOST: ",
"USER: " and "PASSWORD: " lines of a Storage Box subaccount (WebDAV on). Neither value is printed.
#>
param(
    [Parameter(Mandatory)][string]$TokenFile,
    [Parameter(Mandatory)][string]$StorageFile,
    [string]$Type = 'ccx23',
    [string]$Location = 'fsn1',
    [string]$Image = 'ubuntu-26.04',
    [string[]]$Controls = @('v1.2.69'),
    # measure published releases instead of building: this version (e.g. 1.2.76) against
    # -Controls given as versions (1.2.69 1.0.1 ...), downloaded and checked by fetch.sh
    [string]$Published,
    # a shorter run: -Scenarios many (no transfer), -Clients rookparcel,qbittorrent (controls
    # are always measured)
    [string[]]$Scenarios = @('transfer', 'many'),
    [string[]]$Clients = @(),
    [double]$DeadlineHours = 6,
    [string]$RunName = "$(Get-Date -Format yyyy-MM-dd-HHmm)-hetzner-$Type",
    [string]$WorkDir = (Join-Path $env:TEMP "rookparcel-bench-$RunName"),
    # skip the benchmark: check the upload and the self-delete on a cheap server (-Type cx23)
    [switch]$SelfTest
)
$ErrorActionPreference = 'Stop'
$here = $PSScriptRoot

function Get-Field([string]$file, [string]$label, [string]$pattern = '\S+') {
    $m = [regex]::Match((Get-Content -Raw $file), "(?m)^$label\s*($pattern)")
    if (-not $m.Success) { throw "no '$label' in $file" }
    $m.Groups[1].Value
}
$main = Get-Field $TokenFile 'MAIN TOKEN:' '[A-Za-z0-9]{64}'
$bench = Get-Field $TokenFile 'BENCH TOKEN:' '[A-Za-z0-9]{64}'
$sHost = Get-Field $StorageFile 'HOST:'; $sUser = Get-Field $StorageFile 'USER:'; $sPass = Get-Field $StorageFile 'PASSWORD:'

function Invoke-Hcloud([string]$Method, [string]$Path, $Body) {
    $a = @{ Method = $Method; Uri = "https://api.hetzner.cloud$Path"; Headers = @{ Authorization = "Bearer $main" }
            ContentType = 'application/json' }
    if ($Body) { $a.Body = $Body | ConvertTo-Json -Depth 6 }
    Invoke-RestMethod @a
}

New-Item -ItemType Directory -Force $WorkDir | Out-Null
$key = Join-Path $WorkDir 'id_bench'
ssh-keygen -q -t ed25519 -N '' -C "rookparcel-bench $RunName" -f $key
$ssh = @('-i', $key, '-o', 'StrictHostKeyChecking=accept-new', '-o', "UserKnownHostsFile=$WorkDir\known_hosts",
         '-o', 'ConnectTimeout=15', '-o', 'ServerAliveInterval=30', '-o', 'BatchMode=yes')

$keyId = (Invoke-Hcloud POST /v1/ssh_keys @{ name = $RunName; public_key = (Get-Content -Raw "$key.pub").Trim() }).ssh_key.id
$server = (Invoke-Hcloud POST /v1/servers @{
    name = $RunName; server_type = $Type; image = $Image; location = $Location; ssh_keys = @($keyId)
    labels = @{ purpose = 'rookparcel-bench' }; public_net = @{ enable_ipv4 = $true; enable_ipv6 = $true } }).server
"server $($server.id) $Type in $Location, run $RunName"
$armed = $false
try {
    do { Start-Sleep 5; $s = (Invoke-Hcloud GET "/v1/servers/$($server.id)").server } while ($s.status -ne 'running')
    $ip = $s.public_net.ipv4.ip
    for ($i = 0; $i -lt 30; $i++) { ssh @ssh "root@$ip" true 2>$null; if ($LASTEXITCODE -eq 0) { break }; Start-Sleep 5 }
    scp @ssh (Join-Path $here 'setup.sh') (Join-Path $here 'build.sh') (Join-Path $here 'fetch.sh') (Join-Path $here 'run.sh') `
        (Join-Path $here 'finish-hetzner.sh') "root@${ip}:/root/"
    if ($LASTEXITCODE) { throw 'scp failed' }
    # the env file goes over stdin: the values never appear on a command line or in cloud-init data
    @"
HCLOUD_TOKEN='$bench'
STORAGE_HOST='$sHost'
STORAGE_USER='$sUser'
STORAGE_PASS='$sPass'
RUN_NAME='$RunName'
SSH_KEY_ID='$keyId'
"@ | ssh @ssh "root@$ip" 'umask 077; tr -d "\r" > /etc/bench-finish.env'
    $minutes = [int]($DeadlineHours * 60)
    ssh @ssh "root@$ip" "sed -i 's/\r$//' /root/*.sh && install -m 700 /root/finish-hetzner.sh /usr/local/sbin/bench-finish && systemd-run --quiet --unit=bench-deadline --on-active=${minutes}m /usr/local/sbin/bench-finish deadline && systemctl list-timers bench-deadline --no-pager | head -2"
    if ($LASTEXITCODE) { throw 'arming the deadline failed' }
    $armed = $true
    "deadline armed: the server deletes itself after $DeadlineHours h at the latest"
    if ($SelfTest) {
        # no benchmark: the finish step's upload and self-delete, checked from here, called the
        # way a real run calls it (run.sh as the sudo user `bench`; root alone would not show a
        # check that fails for `bench`, as the first CCX23 run's did)
        ssh @ssh "root@$ip" 'bash /root/setup.sh --user-only > /root/setup.log 2>&1 && cp /root/run.sh /home/bench/ && chown bench:bench /home/bench/run.sh && su - bench -c "bash ~/run.sh --finish"; echo self-test exit $?'
        $gone = $false
        for ($i = 0; $i -lt 24 -and -not $gone; $i++) {
            Start-Sleep 5
            try { Invoke-Hcloud GET "/v1/servers/$($server.id)" | Out-Null } catch { $gone = $true }
        }
        $keys = (Invoke-Hcloud GET '/v1/ssh_keys').ssh_keys | Where-Object id -eq $keyId
        $auth = @{ Authorization = 'Basic ' + [Convert]::ToBase64String([Text.Encoding]::ASCII.GetBytes("${sUser}:$sPass")) }
        $list = Invoke-WebRequest -CustomMethod PROPFIND -Uri "https://$sHost/$RunName/" -Headers ($auth + @{ Depth = '1' })
        $files = [regex]::Matches($list.Content, '<d:href>([^<]+)</d:href>', 'IgnoreCase') | ForEach-Object { $_.Groups[1].Value }
        "self-test: server deleted: $gone; SSH key deleted: $(-not $keys); uploaded: $($files -join ' ')"
        return
    }
    ssh @ssh "root@$ip" 'bash /root/setup.sh > /root/setup.log 2>&1; echo setup exit $?; cp /root/build.sh /root/fetch.sh /root/run.sh /home/bench/ && chown bench:bench /home/bench/*.sh'
    ssh @ssh "bench@$ip" 'sudo -n test -x /usr/local/sbin/bench-finish'
    if ($LASTEXITCODE) { 'WARNING: bench cannot run bench-finish; only the deadline will end this server' }
    $labels = ($Controls | ForEach-Object { $_ -replace '^v', '' }) -join ' '
    if ($Published) {
        ssh @ssh "bench@$ip" "bash ~/fetch.sh $Published $labels > ~/build.log 2>&1; r=`$?; echo fetch exit `$r; grep -E 'nox [0-9]|: OK' ~/build.log | grep -v '^+'; exit `$r"
    } else {
        ssh @ssh "bench@$ip" "bash ~/build.sh $($Controls -join ' ') > ~/build.log 2>&1; echo build exit `$?; grep -E 'rookparcel-nox|rustc' ~/build.log | grep -v '^+'"
    }
    if ($Published -and $LASTEXITCODE) {
        ssh @ssh "root@$ip" '/usr/local/sbin/bench-finish fetch-failed'
        throw "fetch.sh failed; build.log uploaded to https://$sHost/$RunName/, server deleting itself"
    }
    $envs = "BENCH_SCENARIOS='$($Scenarios -join ' ')' BENCH_CLIENTS='$($Clients -join ',')'"
    ssh @ssh "bench@$ip" "mkdir -p ~/bench; $envs nohup setsid bash ~/run.sh $labels > ~/bench/run.log 2>&1 < /dev/null & sleep 5; head -3 ~/bench/run.log"
    "started on $ip; results go to https://$sHost/$RunName/ when it ends"
} catch {
    if (-not $armed) {
        "launch failed before the deadline was armed: deleting server $($server.id) and key $keyId"
        Invoke-Hcloud DELETE "/v1/servers/$($server.id)" | Out-Null
        Invoke-Hcloud DELETE "/v1/ssh_keys/$keyId" | Out-Null
    }
    throw
}
