// ----------- CHANGE THIS -------------
// ----------- IP ADDRESS --------------
// -------------------------------------

pub static IPADDR: &str = "127.0.0.1:5455";

// CR, LF, CRLF ---------------

pub static CRLF: &str = "\r\n";
pub static CR: &str = "\r";
pub static LF: &str = "\n";

// HTTP VERSIONS -------------------------------
pub static HTTP11: &str = "HTTP/1.1";
pub static HTTP2: &str = "HTTP/2";
pub static HTTP3: &str = "HTTP/3";

// FILES HARDCODE ------------------------------

pub static ERRORLOGS: &str = "C:/Users/randr/Documents/Rust/HTTPRemake/src/logs/error_logs.txt";
pub static MAINSITE: &str = "C:/Users/randr/Documents/Rust/HTTPRemake/src/html/main.html";

// ERRORS ---------------------------------

pub static ERROR404: &str = "404 NOT FOUND";
pub static ERROR500: &str = "500 INTERNAL SERVER ERROR";
pub static STREAMREADERROR: &str = "FAILED TO READ FROM STREAM: ";
pub static UNABLETOCONNECT: &str = "UNNABLE TO CONNECT: ";

// STATUSES --------------------------------

pub static STATUS200: &str = "200 OK";

// CONTENT TYPES -------------------------------

pub static TEXTPLAIN: &str = "text/plain";
pub static HTML: &str = "text/html";

// PATH CONTENTS ---------------------

pub static TESTE_CONTENT: &str = "Teste!";
pub static NOVO_CONTENT: &str = "Novo!";
pub static PAGENOTFOUND: &str = "PAGE NOT FOUND :(";