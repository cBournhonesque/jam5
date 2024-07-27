trunk build --release --dist target
zip --recurse-paths 'cycles_io-web.zip' 'target'
BUTLER_API_KEY=FGBKDjQgkMCcqM4PHwBdJ9Vj9NX0Bl3ZpLex1B1s ./butler push --fix-permissions --userversion=v0.0.5 cycles_io-web.zip cbournhonesque/cyclesio:web