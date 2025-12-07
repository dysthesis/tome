# Tome - a command-line, plaintext-first citation manager

Tome manages citation in the command-line, using a plain-text file to keep track
of entries. The goal of this is to

- maximise interoperability with other tools, such as Git to synchronise data,
- keep data in a future-proof format, and
- reproducibility, ensuring the reproducibility of the documents.

Tome keeps a `Tome.toml`, which defines the list of citation entries, and a
`Tome.lock`, which maintains the hashes of the relevant artifacts.

## Architecture

Tome is fundamentally a "package manager" for citations. Its responsibility is
to manage a library of citations with respect to the goals stated above. 
However, observe that the life cycle of a citation is as follows:

1. the user enters in an identifier, and optionally, their own metadata,
2. the identifier is parsed,
3. the metadata and artifacts associated to the parsed identifier are fetched, 
   and
4. the results are recorded and hashed, then stored.

Additionally, there is step

5. export the contents as a different format, _e.g._ BibTeX, Hayagriva, _etc._

Tome does not handle steps 2, 3, and 5 above. Instead, it provides an interface 
to communicate with external tools which does. Parsers, fetchers, and exporters
are defined as command-line scripts, which are executed with the necessary
arguments, and the results parsed once terminated. Tome merely provides the
primitives to compose with these external tools.
