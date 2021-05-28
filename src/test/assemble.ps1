if($args[0].Length -ne 0){
    $output = $args[0].replace("asm","nes")
    asm6f_64.exe $args[0] $output
    Write-Host "Displaying content of the assembled file"
    Get-Content $output -AsByteStream | Format-Hex
}
else{
    Write-Host "You should specify the assembly file to assemble."
}
