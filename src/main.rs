macro_rules! brainfuck {
    //Entry
    (input=$inp:literal; mem_size=$mem:literal; $($tt:tt)*) => {
        {
            struct BF<'a> {
                _input: core::str::Bytes<'a>,
                _mem: [u8; $mem],
                _point: usize
            }
            let mut _bf = BF{
                _input: $inp.bytes(),
                _mem: [0u8; $mem],
                _point: 0,
            };
            brainfuck!(@ _bf; [] $($tt)*);
        }
    };
    (mem_size=$mem:literal; input=$inp:literal; $($tt:tt)*) =>{brainfuck!(input=$inp; mem_size=$mem; $($tt)*)};

    //Stack is empty
    (@ $runner:ident; [] + $($tt:tt)*) => {
        brainfuck!(@ $runner; [+] $($tt)*);
    };
    (@ $runner:ident; [] - $($tt:tt)*) => {
        brainfuck!(@ $runner; [-] $($tt)*);
    };
    (@ $runner:ident; [] < $($tt:tt)*) => {
        brainfuck!(@ $runner; [<] $($tt)*);
    };
    (@ $runner:ident; [] << $($tt:tt)*) => {
        brainfuck!(@ $runner; [<<] $($tt)*);
    };
    (@ $runner:ident; [] > $($tt:tt)*) => {
        brainfuck!(@ $runner; [>] $($tt)*);
    };
    (@ $runner:ident; [] >> $($tt:tt)*) => {
        brainfuck!(@ $runner; [>>] $($tt)*);
    };
    //Special tokens, stack should always be empty on call
    (@ $runner:ident; [] [ $($loop:tt)* ] $($tt:tt)*) => {
        while $runner._mem[$runner._point] != 0 {
            brainfuck!(@ $runner; [] $($loop)*);
        }
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [] . $($tt:tt)*) => {
        print!("{}", $runner._mem[$runner._point] as char);
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [] .. $($tt:tt)*) => {
        print!("{}", $runner._mem[$runner._point] as char);
        brainfuck!(@ $runner; [] . $($tt)*);
    };
    (@ $runner:ident; [] , $($tt:tt)*) => {
        match $runner._input.next() {
            Some(v) => {
                $runner._mem[$runner._point] = v;
            },
            None => panic!("Unexpected end of input"),
        };
        brainfuck!(@ $runner; [>] $($tt)*);
    };

    //Next is self
    (@ $runner:ident; [+ $($bf:tt)*] + $($tt:tt)*) => {
        brainfuck!(@ $runner; [+ + $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [- $($bf:tt)*] - $($tt:tt)*) => {
        brainfuck!(@ $runner; [- - $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [> $($bf:tt)*] > $($tt:tt)*) => {
        brainfuck!(@ $runner; [> > $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [>> $($bf:tt)*] > $($tt:tt)*) => {
        brainfuck!(@ $runner; [> > > $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [> $($bf:tt)*] >> $($tt:tt)*) => {
        brainfuck!(@ $runner; [> > > $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [>> $($bf:tt)*] >> $($tt:tt)*) => {
        brainfuck!(@ $runner; [> > > > $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [< $($bf:tt)*] < $($tt:tt)*) => {
        brainfuck!(@ $runner; [< < $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [<< $($bf:tt)*] < $($tt:tt)*) => {
        brainfuck!(@ $runner; [< < < $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [< $($bf:tt)*] << $($tt:tt)*) => {
        brainfuck!(@ $runner; [< < < $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [<< $($bf:tt)*] << $($tt:tt)*) => {
        brainfuck!(@ $runner; [< < < < $($bf)*] $($tt)*);
    };

    //Next is inverse
    (@ $runner:ident; [+ $($bf:tt)*] - $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [- $($bf:tt)*] + $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [> $($bf:tt)*] < $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [>> $($bf:tt)*] << $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [> $($bf:tt)*] << $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] < $($tt)*);
    };
    (@ $runner:ident; [>> $($bf:tt)*] < $($tt:tt)*) => {
        brainfuck!(@ $runner; [> $($bf)*] $($tt)*);
    };
    (@ $runner:ident; [< $($bf:tt)*] > $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [<< $($bf:tt)*] >> $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] $($tt)*);
    };
    (@ $runner:ident; [< $($bf:tt)*] >> $($tt:tt)*) => {
        brainfuck!(@ $runner; [$($bf)*] > $($tt)*);
    };
    (@ $runner:ident; [<< $($bf:tt)*] > $($tt:tt)*) => {
        brainfuck!(@ $runner; [< $($bf)*] $($tt)*);
    };

    //Next is new token
    (@ $runner:ident; [+ $($bf:tt)*] $($tt:tt)*) => {
        $runner._mem[$runner._point] = $runner._mem[$runner._point].wrapping_add(stringify!(+ $($bf)*).chars().filter(|c| !c.is_whitespace()).count() as u8);
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [- $($bf:tt)*] $($tt:tt)*) => {
        $runner._mem[$runner._point] = $runner._mem[$runner._point].wrapping_sub(stringify!(- $($bf)*).chars().filter(|c| !c.is_whitespace()).count() as u8);
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [> $($bf:tt)*] $($tt:tt)*) => {
        $runner._point = ($runner._point + stringify!(> $($bf)*).chars().filter(|c| !c.is_whitespace()).count()) % $runner._mem.len();
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [>> $($bf:tt)*] $($tt:tt)*) => {
        $runner._point = ($runner._point + stringify!(>> $($bf)*).chars().filter(|c| !c.is_whitespace()).count()) % $runner._mem.len();
        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [< $($bf:tt)*] $($tt:tt)*) => {
        let left = stringify!(< $($bf)*).chars().filter(|c| !c.is_whitespace()).count();
        match $runner._point.checked_sub(left) {
            Some(v) => {
                $runner._point = v;
            }
            None => {
                $runner._point = $runner._mem.len() - (left - $runner._point);
            }
        }

        brainfuck!(@ $runner; [] $($tt)*);
    };
    (@ $runner:ident; [<< $($bf:tt)*] $($tt:tt)*) => {
        let left = stringify!(<< $($bf)*).chars().filter(|c| !c.is_whitespace()).count();
        match $runner._point.checked_sub(left) {
            Some(v) => {
                $runner._point = v;
            }
            None => {
                $runner._point = $runner._mem.len() - (left - $runner._point);
            }
        }

        brainfuck!(@ $runner; [] $($tt)*);
    };

    //Exit
    () => {};
    (@ $runner:ident; []) => {};

    //literal matches above here
    //Next is non-token
    (@ $runner:ident; [] $_:tt $($tt:tt)*) => {
        brainfuck!(@ $runner; [] $($tt)*);
    };
}

fn main() {
    brainfuck! {
        input="";
        mem_size=3000;
        +++++ +++++             (initialize counter (cell #0) to 10)
        [                       (use loop to set the next four cells to 70/100/30/10)
            > +++++ ++          (    add  7 to cell #1)
            > +++++ +++++       (    add 10 to cell #2 )
            > +++               (    add  3 to cell #3)
            > +                 (    add  1 to cell #4)
            <<<< -              (    decrement counter (cell #0))
        ]
        > ++ .                  (print 'H')
        > + .                   (print 'e')
        +++++ ++ .              (print 'l')
        .                       (print 'l')
        +++ .                   (print 'o')
        > ++ .                  (print ' ')
        << +++++ +++++ +++++ .  (print 'W')
        > .                     (print 'o')
        +++ .                   (print 'r')
        ----- - .               (print 'l')
        ----- --- .             (print 'd')
        > + .                   (print bang)
        > .                     (print '\n')
    };
}
