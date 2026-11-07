# Exhaustive test build profile

Bug #53 records the cost of the complete stored-archive regression suite. The
714-case linear-pattern suite took 270.66 seconds in the default debug test
profile. The same frozen packet and all eight tests took 71.52 seconds at
optimization level 1 and 70.02 seconds at level 3 on Windows x64 with Rust 1.99.0.
The level-1 result is about 3.8 times faster; level 3 adds little benefit.

The workspace test profile therefore uses level 1. Debug assertions and integer
overflow checks remain explicitly enabled. A test checks both properties,
including a runtime checked addition whose overflow must panic. No test,
truncated prefix, trailing-byte case, independent expected value, malformed-input
check or concurrency assertion was removed. Fixture bytes and acceptance
thresholds remain unchanged. Production build profiles are unchanged.

The initial timing comparison used the in-progress public API working tree. A
full local gate on the private PR baseline checks the final committed profile;
its results are retained in `test-performance-verification.json`. The full
platform matrix, including ThreadSanitizer, must pass on the updated PR commit.
Timing is test execution time, excluding compilation. Peak memory was not measured.

The public API work was preserved separately while this validation change was
integrated. Humanizer was unavailable; prose remains for maintainer review.
