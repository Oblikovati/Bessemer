# Source reader-selection refactor

The replay driver neared its file-size limit while new documented models were
being added. Reader selection now lives in a separate module; dispatching tests
and executing replay remain in the driver. The ordered selector still handles
early profiles, deferred specialized callbacks, final replacement profiles and
the default provider in their original order.

Before extraction, owned fixtures captured 2,776 cases over 37 flag inputs,
including write controls and an unknown flag. They cover each individual input,
all ordered pairs and write-decorated pairs. The fixture retains 27 reader
profiles, two captured callback-input sets and the original driver hash.
It was committed before the refactor.

Both the original driver and the extracted selector matched every owned result.
The updated driver was then checked against the same results using its actual
callbacks. The default full gate also replays the selector with captured provider
inputs. That keeps the ordering baseline stable when later features explicitly
add dependencies to a provider; it does not silently regenerate old expectations.
The selector itself neither imports Source models nor modifies execution flags.

The [verification record](reader-selection-verification.json) retains fixture,
driver and validation hashes. This structural change modifies no production
library behavior, parser acceptance, native denominator or byte-coverage credit.
The full integrated replay passed305 checks in509.985seconds and exited
successfully after snapshot recording and cleanup. Publication remains separate.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
