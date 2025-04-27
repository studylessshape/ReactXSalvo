try {
    sta --help | Out-Null
}
catch [System.Management.Automation.CommandNotFoundException] {
    Write-Output "[npm] Installing swagger-typescript-api global..."
    npm install swagger-typescript-api -g
}
sta generate -p http://localhost:5701/api-doc/openapi.json -o ./src/services --modular