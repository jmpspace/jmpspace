& elm make .\src\Main.elm

$outputFile = ".\index.html"

$outputContents = Get-Content $outputFile

$outputContents = $outputContents -replace '/\* SHIM Native.BinaryArrayBuffer \*/', 'var _johnpmayer$jmpspace$Native_Binary_ArrayBuffer = _johnpmayer$binary$Native_Binary_ArrayBuffer'
$outputContents = $outputContents -replace '/\* SHIM Native.WebSocket \*/', 'var _johnpmayer$jmpspace$Native_WebSocket = _elm_lang$websocket$Native_WebSocket'

$outputContents | Set-Content  $outputFile