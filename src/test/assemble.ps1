asm6f_64.exe "test.asm" "test.nes"
Write-Host "Displaying content of the assembled file"
Get-Content "./test.nes" -Encoding Byte `
    -ReadCount 16 | ForEach-Object {
    $output = ""
    foreach ( $byte in $_ ) {
        $output += "{0:X2} " -f $byte
    }
    $output
}