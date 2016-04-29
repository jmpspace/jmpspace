
module Binary exposing (ArrayBuffer, encodeBase64, decodeBase64)

-- non-instantiate-able type
type ArrayBuffer = _ArrayBuffer ArrayBuffer

encodeBase64 : ArrayBuffer -> String
encodeBase64 = Native.Binary.encodeBase64

decodeBase64 : String -> ArrayBuffer
decodeBase64 : Native.Binary.decodeBase64
