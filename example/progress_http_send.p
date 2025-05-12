
/* Define the external Rust DLL procedure */
PROCEDURE perform_http_request EXTERNAL "librust_http_client_dll.so" CDECL:
    DEFINE INPUT PARAMETER cUrl AS CHARACTER.
    DEFINE INPUT PARAMETER cMethod AS CHARACTER.
    DEFINE INPUT PARAMETER cHeaders AS CHARACTER.
    DEFINE INPUT PARAMETER cBody AS CHARACTER.
    DEFINE OUTPUT PARAMETER cStatusCode AS SHORT.
    DEFINE OUTPUT PARAMETER cErrorCode AS SHORT.
    DEFINE RETURN PARAMETER mResponse AS MEMPTR.
END PROCEDURE.
PROCEDURE free_response EXTERNAL "librust_http_client_dll.so" CDECL:
    DEFINE INPUT PARAMETER mResponse AS MEMPTR.
END PROCEDURE.

DEFINE VARIABLE cUrl AS CHARACTER.
DEFINE VARIABLE cMethod AS CHARACTER.
DEFINE VARIABLE cHeaders AS CHARACTER.
DEFINE VARIABLE cBody AS CHARACTER.
DEFINE VARIABLE mResponse AS MEMPTR.
DEFINE VARIABLE cResponse AS CHARACTER.
DEFINE VARIABLE cStatusCode AS int.
DEFINE VARIABLE cErrorCode AS int.
def var iStartTime as int.
def var iDuration as int.
etime(yes).

/* Set request details */
cUrl = "https://httpbin.org/get".
cMethod = "GET".
cHeaders = "Accept: application/json".
cBody = "".
cStatusCode = 0.
cErrorCode = 0.

iStartTime = etime.

/* Call the Rust DLL */
RUN perform_http_request(cUrl, cMethod, cHeaders, cBody, OUTPUT cStatusCode, OUTPUT cErrorCode, OUTPUT mResponse).

iDuration = etime - iStartTime.

/* Check for errors and convert response to CHARACTER */
IF cErrorCode <> 0 THEN
    MESSAGE "Error:" cErrorCode VIEW-AS ALERT-BOX.
ELSE DO:
    /* Convert MEMPTR to CHARACTER */
    cResponse = GET-STRING(mResponse, 1).
    MESSAGE "Status:" cStatusCode "Response:" cResponse.
END.

/* Log profiling data */
OUTPUT TO "progress_profile.log".
PUT "DLL Call Duration: " iDuration " ms" SKIP.
OUTPUT CLOSE.

/* Free the response */
RUN free_response(mResponse).
