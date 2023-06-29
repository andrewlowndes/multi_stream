use futures::{
    stream::{Fuse, StreamExt},
    Stream,
};
use paste::paste;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

//create a macro_rules macro for constructing structs based on the varying typed arguments so we can aggreate streams of different types whilst keeping the types intact
macro_rules! multi_stream_struct {
    ($name: ident, $($items:ident),+) => {
        paste! {
            #[pin_project::pin_project]
            pub struct $name<
                $(
                    [<Stream $items>],
                )*
            >
            where
                $(
                    [<Stream $items>]: Stream,
                    [<Stream $items>]::Item: Clone,
                )*
            {
                $(
                    #[pin]
                    [<stream_ $items:snake>]: Fuse<[<Stream $items>]>,
                    [<last_ $items:snake>]: Option<[<Stream $items>]::Item>,
                )*
            }

            impl<
                $(
                    [<Stream $items>],
                )*
            > $name <
                    $(
                        [<Stream $items>],
                    )*
                >
            where
                $(
                    [<Stream $items>]: Stream,
                    [<Stream $items>]::Item: Clone,
                )*
            {
                #[allow(clippy::too_many_arguments)]
                pub fn new(
                    $(
                        [<stream_ $items:snake>]: [<Stream $items>],
                    )*
                ) -> Self {
                    Self {
                        $(
                            [<stream_ $items:snake>]: [<stream_ $items:snake>].fuse(),
                            [<last_ $items:snake>]: None,
                        )*
                    }
                }
            }

            impl<
                    $(
                        [<Stream $items>],
                    )*
                >
                    Stream for $name <
                    $(
                        [<Stream $items>],
                    )*
                >
            where
                $(
                    [<Stream $items>]: Stream,
                    [<Stream $items>]::Item: Clone,
                )*
            {
                type Item = (
                    $(
                        Option<[<Stream $items>]::Item>,
                    )*
                );

                fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                    let this = self.project();

                    let mut pending = true;
                    let mut done = true;

                    $(
                        match this.[<stream_ $items:snake>].poll_next(cx) {
                            Poll::Ready(Some(val)) => {
                                done = false;
                                pending = false;
                                this.[<last_ $items:snake>].replace(val);
                            },
                            Poll::Ready(None) => {},
                            Poll::Pending => {
                                done = false;
                            },
                        }
                    )*

                    if done {
                        Poll::Ready(None)
                    } else if pending {
                        Poll::Pending
                    } else {
                        let result = (
                            $(
                                this.[<last_ $items:snake>].clone(),
                            )*
                        );

                        Poll::Ready(Some(result))
                    }
                }
            }
        }
    }
}

multi_stream_struct!(MultiStream2, A, B);
multi_stream_struct!(MultiStream3, A, B, C);
multi_stream_struct!(MultiStream4, A, B, C, D);
multi_stream_struct!(MultiStream5, A, B, C, D, E);
multi_stream_struct!(MultiStream6, A, B, C, D, E, F);
multi_stream_struct!(MultiStream7, A, B, C, D, E, F, G);
multi_stream_struct!(MultiStream8, A, B, C, D, E, F, G, H);
multi_stream_struct!(MultiStream9, A, B, C, D, E, F, G, H, I);
multi_stream_struct!(MultiStream10, A, B, C, D, E, F, G, H, I, J);
multi_stream_struct!(MultiStream11, A, B, C, D, E, F, G, H, I, J, K);
multi_stream_struct!(MultiStream12, A, B, C, D, E, F, G, H, I, J, K, L);

#[macro_export]
macro_rules! multi_stream {
    () => {
        panic!("No streams to multi_stream");
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::MultiStream2::new($a, $b)
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::MultiStream3::new($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::MultiStream4::new($a, $b, $c, $d)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::MultiStream5::new($a, $b, $c, $d, $e)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::MultiStream6::new($a, $b, $c, $d, $e, $f)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
        $crate::MultiStream7::new($a, $b, $c, $d, $e, $f, $g)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => {
        $crate::MultiStream8::new($a, $b, $c, $d, $e, $f, $g, $h)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => {
        $crate::MultiStream9::new($a, $b, $c, $d, $e, $f, $g, $h, $i)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => {
        $crate::MultiStream10::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr) => {
        $crate::MultiStream11::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr) => {
        $crate::MultiStream12::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l)
    };
}
