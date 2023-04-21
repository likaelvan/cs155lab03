use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(expr) =>  Value::IntValue(eval_arith_expr(expr)),
        BoolExpr(expr) => Value::BoolValue(eval_bool_expr(expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {      
        BinArithExpr {left, right, op} => match op {
            AddOp => eval_arith_expr(*left) + eval_arith_expr(*right),
            SubOp =>eval_arith_expr(*left) - eval_arith_expr(*right),
            MulOp => eval_arith_expr(*left) * eval_arith_expr(*right),
            IntDivOp => eval_arith_expr(*left) / eval_arith_expr(*right),
        },
        IntLit(num) => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {    
        ArithCmpExpr {left, right, op} => match op {
            LtOp => eval_arith_expr(*left) < eval_arith_expr(*right),
            LteOp => eval_arith_expr(*left) <= eval_arith_expr(*right) ,
            GtOp => eval_arith_expr(*left) > eval_arith_expr(*right),
            GteOp => eval_arith_expr(*left) >= eval_arith_expr(*right),
            ArithEqOp => eval_arith_expr(*left) == eval_arith_expr(*right),
            ArithNeqOp => eval_arith_expr(*left) != eval_arith_expr(*right),
        },   
        BinBoolExpr {left, right, op} => match op {
            AndOp => eval_bool_expr(*left) && eval_bool_expr(*right),
            OrOp => eval_bool_expr(*left) || eval_bool_expr(*right),
            BoolEqOp => eval_bool_expr(*left) == eval_bool_expr(*right),
            BoolNeqOp => eval_bool_expr(*left) != eval_bool_expr(*right),
        },
        NotExpr(bool_expr) => !(eval_bool_expr(*bool_expr)),
        BoolLit(tf) => tf,
    }
}

fn main() {}

mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }

    #[test]
    fn test_addop_1() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(10)), right: Box::new(IntLit(20)), op: AddOp } );
        let answer = IntValue(30);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_addop_2() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(10)), right: Box::new(IntLit(-20)), op: AddOp } );
        let answer = IntValue(-10);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_addop_3() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(-10)), right: Box::new(IntLit(-20)), op: AddOp } );
        let answer = IntValue(-30);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_subop_1() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(50)), right: Box::new(IntLit(30)), op: SubOp } );
        let answer = IntValue(20);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_subop_2() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(50)), right: Box::new(IntLit(-30)), op: SubOp } );
        let answer = IntValue(80);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_subop_3() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(-50)), right: Box::new(IntLit(-30)), op: SubOp } );
        let answer = IntValue(-20);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_mulop_1() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(4)), right: Box::new(IntLit(5)), op: MulOp } );
        let answer = IntValue(20);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_mulop_2() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(-4)), right: Box::new(IntLit(5)), op: MulOp } );
        let answer = IntValue(-20);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_mulop_3() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(-4)), right: Box::new(IntLit(-5)), op: MulOp } );
        let answer = IntValue(20);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_intdivop() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(50)), right: Box::new(IntLit(5)), op: IntDivOp } );
        let answer = IntValue(10);
        assert_eq!(eval(expr), answer);  
    }

   #[test]
    fn test_ltop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (LtOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_ltop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: (LtOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_lteop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(2)), op: (LteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_lteop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (LteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_lteop_3() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: (LteOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_gtop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: (GtOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_gtop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (GtOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_gteop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(2)), op: (GteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_gteop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: (GteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_gteop_3() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (GteOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_aritheqop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(1)), op: (ArithEqOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_aritheqop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (ArithEqOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_arithneqop_1() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(1)), op: (ArithNeqOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_arithneqop_2() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(1)), right: Box::new(IntLit(2)), op: (ArithNeqOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
   
   #[test]
    fn test_andop_1() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: AndOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_andop_2() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_andop_3() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: AndOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_andop_4() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: AndOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_orop_1() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: OrOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_orop_2() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: OrOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_orop_3() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: OrOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_orop_4() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: OrOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_booleqop() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: BoolEqOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_boolneqop_1() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: BoolNeqOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_boolneqop_2() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_not_1() {
        let expr = BoolExpr(NotExpr(Box::new(BoolLit(true))));
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_not_2() {
        let expr = BoolExpr(NotExpr(Box::new(BoolLit(false))));
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

}