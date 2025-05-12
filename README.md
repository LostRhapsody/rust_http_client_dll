# Rust HTTP Client DLL

A Rust library meant to serve as an executable HTTP client.

## Use Case

This library can be built as an executable and referenced in external programs. Why? Some platforms don't have robust HTTP clients. This alllows other platforms to utilize Rust's very robust HTTP client crate, reqwest.

## Unsafe

This library uses mostly unsafe code so it can be referenced externally using C bindings. This is to make it more compatible as most platforms don't have Rust bindings. The code itself is not all that unsafe, but contracts between this DLL and the caller are very strict.

## Contracts

The caller process must call free_response after reading the pointer we return that points to the response data. If the caller attempts to clean up this pointer itself, or forgets to call free_response, undefined behavior is guarunteed. If everything follows the contract, it's a fairly simple and safe process. Essentially, not much more 'unsafe' than your average HTTP client. Still, not as safe or robust as a pure Rust client.

## Recommendations

If your platform does not have a reliable HTTP client, this is a viable option. However, if missused, the chance of UB is high; only use under tightly controlled environments. A wrapper, to ensure it's always used properly, is encouraged.
