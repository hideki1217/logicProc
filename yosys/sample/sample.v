module full_adder (xin, yin, cin, sout, cout);
 input xin, yin, cin;
 output sout, cout;
 assign sout = (xin ^ yin) ^ cin;
 assign cout = (xin & yin) | ((xin ^ yin) & cin);
endmodule