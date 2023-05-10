1 while dup 100 <= do
  dup 15 mod 0 == do
	  122 122 117 66 122 122 105 70
	  0 while dup 8 < do
      swap
      putunicode
      1 + 
	  end
	  pop
  else
    dup 5 mod 0 == do
      122 122 117 66
      0 while dup 4 < do
        swap
        putunicode
        1 +
      end
      pop
    else 
      dup 3 mod 0 == do
        122 122 105 70
        0 while dup 4 < do
          swap
          putunicode
          1 +
        end
        pop
      else
         dup put
      end
    end 
  end
  blank
  1 +
end
