#!/usr/bin/env ruby
puts "["
(0...10000).each do |n|
  s = n.to_s
  word = 0;
  (s.length - 1).downto(0).each do |i|
    word = (word << 8) | s[i].ord
  end
  printf "(#{s.length}, 0x%08x),\n", word
end
puts "];"
