# rustc_static_groups

This doesn't work, and probably will never work. To make it work we would have to store a (phf) map of strings to
function references and store them in the module somehow (not possible in rustc atm). Then the final product would have to collect references
to this map somehow (which isn't something that's exposed through an API in rustc) and merge and look through them.