#!/usr/bin/env ruby
puts "["
(0...10000).each do |n|
  s = sprintf '%04d', n
  word = 0
  (0...s.length).each do |i|
    word = (word << 8) | s[i].ord
  end
  #word |= (s.length << 24)
  printf "0x%08x,\n", word
end
puts "];"
