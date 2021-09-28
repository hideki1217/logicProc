module full_adder (A,B,X,carry);
 input [3:0] A, B;
 output [3:0] X;
 output carry;
 assign {carry,X} = A + B;
endmodule