#![allow(unused)]

// let code = "x:= 55; y:= 5; z:= x + 2 + y";
// let code = "x:= 55; while (5 + 4) = 5 - 3 do {x:=x+5;skip};skip;skip";
// let code = "x:= 55; while (5 + 4) = 5 - 3 do {x:=x+5;skip};if(x<=22)and(55=3)then skip else skip;skip";
// let code = "if true then if false then x:=1 else x:=2;y:=22 else x:=3";
// let code = "if (not false and true) then skip else skip";
pub const FACTORIAL: &str = "y:=25;x:=1;while(not(y=0)) do {x:=x*y;y:=y-1;}";
pub const LONG_LOOP: &str = "y:=1000000;x:=1;while(not(y=0)) do {x:=x+y;y:=y-1;}";
pub const INFINITE_LOOP: &str = "while true do skip;"; 
pub const WHILE_FALSE: &str = "x:=1;while false do x:=2;";
pub const INNER_LOOP: &str = r#"
    x:=0; y:=1;
    while x<=1000 do {
        x:=x+10;
        while (y<=10) do
            y:=y+1;
        y:=y*2;
    }"#;
pub const GCD: &str = r#"
    n1 := 814324; n2 := 1532;
    while (not n1 = n2) do 
        if n1 <= n2 then 
            n2:= n2 - n1;
        else
            n1:= n1 - n2;
    gcd:=n1;
"#;
pub const TEST_REPEAT_UNTIL: &str = r#"
    x:=1023;
    repeat x-=110; until x<333;
"#;
pub const TEST_FOR_LOOP: &str = r#"
    y:=1;
    for(x:=1; x<=500000; x:=x+1){
        y:=y+x;
    }
"#;