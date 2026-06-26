use maud::{Markup, html};
use std::sync::LazyLock;

const BLINK_COVER:      &str = "blink.jpg";
const NOTESFROM_COVER:  &str = "notesfromunder.jpg";
const HOUSE_COVER:      &str = "house.jpg";
const TOSELL_COVER:     &str = "tosell.jpg";
const UNDERSTAND_COVER: &str = "understandingprogrammer.jpg";
const NINETEEN84_COVER: &str = "1984.jpg";
const THETRIPLE_COVER:  &str = "thetriple.jpg";
const EMBRSUCK_COVER:   &str = "embracesuck.jpg";
const ALCHEMIST_COVER:  &str = "alchemist.jpg";
const REVINSAN_COVER:   &str = "reverendinsanity.jpeg";

pub fn reading() -> Markup {
    html! {
        .body-container {
            .reading-list {
                h2 { "Currently Reading" }
                ((*REVINSAN_BOOK).showcase())
                ((*ALCHEMIST_BOOK).showcase())
                ((*NOTESFROM_BOOK).showcase())
                ((*HOUSE_BOOK).showcase())
            }
            .reading-list {
                h2 { "Favorites" }
                ((*UNDERSTAND_BOOK).showcase())
                ((*NINETEEN84_BOOK).showcase())
            }
            .reading-list {
                h2 { "Planning to Read" }
                ((*TOSELL_BOOK).showcase())
                ((*THETRIPLE_BOOK).showcase())
            }
            .reading-list {
                h2 { "Previously Read" }
                ((*BLINK_BOOK).showcase())
                ((*EMBRACESUCK_BOOK).showcase())
            }
        }
    }
}

static REVINSAN_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "Reverend Insanity",
    author: "蛊真人",
    cover:  REVINSAN_COVER,
    thoughts: html! {},
});

static EMBRACESUCK_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "Embrace the Suck",
    author: "Brent Gleeson",
    cover:  EMBRSUCK_COVER,
    thoughts: html! {
        p {
            "A great guide to living a fulfilling life. I thought it would be a cringy read,
            but I found it quickly eases readers through its engaging anecdotes and relatability."
        }
    },
});

static ALCHEMIST_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "The Alchemist",
    author: "Paulo Coelho",
    cover:  ALCHEMIST_COVER,
    thoughts: html! {},
});

static NOTESFROM_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "Notes From the Underground",
    author: "Fyodor Dostoevsky",
    cover:  NOTESFROM_COVER,
    thoughts: html! {},
});

static BLINK_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "Blink",
    author: "Malcolm Gladwell",
    cover:  BLINK_COVER,
    thoughts: html! {
        p {
            "A great read! I honestly learned so much about our instincts and our brain's rapid initial processing."
        }
    },
});

static HOUSE_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "House of Leaves",
    author: "Mark Z. Danielewski",
    cover:  HOUSE_COVER,
    thoughts: html! {},
});

static TOSELL_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "To Sell is Human",
    author: "Daniel H. Pink",
    cover:  TOSELL_COVER,
    thoughts: html! {},
});

static THETRIPLE_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "The Triple Package",
    author: "Amy Chua, Jeb Rubenfeld",
    cover:  THETRIPLE_COVER,
    thoughts: html! {},
});

static UNDERSTAND_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "Understanding the Professional Programmer",
    author: "Gerald M. Weinberg",
    cover:  UNDERSTAND_COVER,
    thoughts: {
        html! {
            p {
                "Although this book was written almost 40 years ago,
                it was oddly comforting to see that the same struggles 
                for programmers back then are still relevant today. 
                Thinking about it again, maybe it's more disappointing than comforting. 
                Nonetheless, I recommend it for programmers and managers of them." 
            }
        }
    },
});

static NINETEEN84_BOOK: LazyLock<Book<'static>> = LazyLock::new(|| Book {
    title:  "1984",
    author: "George Orwell",
    cover:  NINETEEN84_COVER,
    thoughts: {
        html! {
            p {
                "Does this one need an explanation?"
            }
        }
    },
});

struct Book<'a> {
    title:  &'a str,
    author: &'a str,
    cover:  &'a str,
    thoughts: Markup,
}
impl<'a> Book<'a> {
    fn showcase(&self) -> Markup {
        let book_cover = format!("/public/images/reading/{}", self.cover);

        html! {
            .book {
                .book-cover {
                    img src=(book_cover) alt=(self.title);
                }
                .book-info {
                    h3 { (self.title) }
                    p { b { "Author: " } (self.author) }
                    (self.thoughts)
                }
            }
        }
    }
}
