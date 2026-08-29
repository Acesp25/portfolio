use maud::{Markup, html};
use std::sync::LazyLock;

use super::images::{Loading, Photo};

pub fn reading() -> Markup {
    html! {
        .body-container {
            .reading-list {
                h2 { "Currently Reading" }
                (REVEREND_INSANITY.showcase())
                (ALCHEMIST.showcase())
                (NOTES_UNDERGROUND.showcase())
                (HOUSE_OF_LEAVES.showcase())
            }
            .reading-list {
                h2 { "Favorites" }
                (UNDERSTANDING_PROGRAMMER.showcase())
                (NINETEEN_EIGHTY_FOUR.showcase())
            }
            .reading-list {
                h2 { "Planning to Read" }
                (TO_SELL_IS_HUMAN.showcase())
                (TRIPLE_PACKAGE.showcase())
            }
            .reading-list {
                h2 { "Previously Read" }
                (BLINK.showcase())
                (EMBRACE_THE_SUCK.showcase())
            }
        }
    }
}

static REVEREND_INSANITY: LazyLock<Book> = LazyLock::new(|| Book {
    title: "Reverend Insanity",
    author: "蛊真人",
    cover: Photo::book("reverendinsanity", "Cover of Reverend Insanity"),
    thoughts: None,
});

static ALCHEMIST: LazyLock<Book> = LazyLock::new(|| Book {
    title: "The Alchemist",
    author: "Paulo Coelho",
    cover: Photo::book("alchemist", "Cover of The Alchemist"),
    thoughts: None,
});

static NOTES_UNDERGROUND: LazyLock<Book> = LazyLock::new(|| Book {
    title: "Notes From the Underground",
    author: "Fyodor Dostoevsky",
    cover: Photo::book("notesfromunder", "Cover of Notes From the Underground"),
    thoughts: None,
});

static HOUSE_OF_LEAVES: LazyLock<Book> = LazyLock::new(|| Book {
    title: "House of Leaves",
    author: "Mark Z. Danielewski",
    cover: Photo::book("house", "Cover of House of Leaves"),
    thoughts: None,
});

static UNDERSTANDING_PROGRAMMER: LazyLock<Book> = LazyLock::new(|| Book {
    title: "Understanding the Professional Programmer",
    author: "Gerald M. Weinberg",
    cover: Photo::book(
        "understandingprogrammer",
        "Cover of Understanding the Professional Programmer",
    ),
    thoughts: Some(html! {
        p {
            "Although this book was written almost 40 years ago, it was oddly comforting "
            "to see that the same struggles for programmers back then are still relevant "
            "today. Thinking about it again, maybe that's more disappointing than "
            "comforting. Either way, I recommend it to programmers and to the people "
            "managing them."
        }
    }),
});

static NINETEEN_EIGHTY_FOUR: LazyLock<Book> = LazyLock::new(|| Book {
    title: "1984",
    author: "George Orwell",
    cover: Photo::book("1984", "Cover of 1984"),
    thoughts: Some(html! {
        p { "Does this one need an explanation?" }
    }),
});

static TO_SELL_IS_HUMAN: LazyLock<Book> = LazyLock::new(|| Book {
    title: "To Sell is Human",
    author: "Daniel H. Pink",
    cover: Photo::book("tosell", "Cover of To Sell is Human"),
    thoughts: None,
});

static TRIPLE_PACKAGE: LazyLock<Book> = LazyLock::new(|| Book {
    title: "The Triple Package",
    author: "Amy Chua, Jed Rubenfeld",
    cover: Photo::book("thetriple", "Cover of The Triple Package"),
    thoughts: None,
});

static BLINK: LazyLock<Book> = LazyLock::new(|| Book {
    title: "Blink",
    author: "Malcolm Gladwell",
    cover: Photo::book("blink", "Cover of Blink"),
    thoughts: Some(html! {
        p {
            "A great read! I learned a lot about our instincts and our brain's rapid "
            "initial processing."
        }
    }),
});

static EMBRACE_THE_SUCK: LazyLock<Book> = LazyLock::new(|| Book {
    title: "Embrace the Suck",
    author: "Brent Gleeson",
    cover: Photo::book("embracesuck", "Cover of Embrace the Suck"),
    thoughts: Some(html! {
        p {
            "A guide to living a fulfilling life. I thought it would be a cringy read, "
            "but it eases you through with engaging anecdotes and a lot of relatability."
        }
    }),
});

struct Book {
    title:  &'static str,
    author: &'static str,
    cover:  Photo,
    thoughts: Option<Markup>,
}

impl Book {
    fn showcase(&self) -> Markup {
        html! {
            .book {
                .book-cover {
                    (self.cover.img(Loading::Lazy))
                }
                .book-info {
                    h3 { (self.title) }
                    p { strong { "Author: " } (self.author) }
                    /*
                     * Deref matters: `&self.thoughts` binds `thoughts` as
                     * `&Markup`, and Maud has no Render impl for a reference
                     * to Markup -- it takes the reference itself.
                     */   
                    @if let Some(thoughts) = &self.thoughts {
                        (*thoughts)
                    }
                }
            }
        }
    }
}
