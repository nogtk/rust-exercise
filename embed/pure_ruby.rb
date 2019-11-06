require 'benchmark'

threads = []

Benchmark.bm(10) do |r|
  r.report "pure_ruby" do
    10.times do
      threads << Thread.new do
        count = 0

        5_000_000.times do
          count += 1
        end

        count
      end
    end

    threads.each do |t|
      puts "Thread finished with count=#{t.value}"
    end

    puts "done!"

  end
end
