# windows
rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}

# Unix
rustc -Vv | grep host | cut -f2 -d' '