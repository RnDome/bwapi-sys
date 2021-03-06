bindgen BWAPI.h -o lib.rs \
  --opaque-type ".+_" \
  --blacklist-type "std.*|__.+|.+_$|Game_v(Send|Print|Draw).*|va_list|.+_t$" \
  --no-layout-tests \
  --no-derive-debug \
  --raw-line "#![allow(improper_ctypes, non_snake_case)]" \
  -- -I../submodules/bwapi-c/include

sed -i -r -- 's/.+\s+(.+)_;/pub struct \1;/' lib.rs
