require 'ffi'
require 'benchmark'

module Hello
  extend FFI::Library
  ffi_lib 'target/release/libembed.dylib'
  attach_function :process, [], :void
end


Benchmark.bm(10) do |r|
  r.report "use_rust" do
    Hello.process
    puts 'done!'
  end
end