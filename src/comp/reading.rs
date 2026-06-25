use maud::{Markup, html};

const BLINK_BOOK: &str = "blink.jpg";
const NOTESFROM_BOOK: &str = "notesfromunder.jpg";
const HOUSE_BOOK: &str = "house.jpg";
const TOSELL_BOOK: &str = "tosell.jpg";
const UNDERSTANDING_BOOK: &str = "understandingprogrammer.jpg";
const NINETEEN84_BOOK: &str = "1984.jpg";

pub fn reading() -> Markup {
    html! {
        .body-container {
            .reading-list {
                h2 { "Currently Reading" }
                (book(
                    NOTESFROM_BOOK,
                    "Notes From the Underground",
                    "Fyodor Dostoevsky",
                    html! {}
                ))
                (book(
                    BLINK_BOOK,
                    "Blink",
                    "Malcolm Gladwell",
                    html! {}
                ))
                (book(
                    HOUSE_BOOK,
                    "House of Leaves",
                    "Mark Z. Danielewski",
                    html! {}
                ))
            }
            .reading-list {
                h2 { "Planning to Read" }
                (book(
                    TOSELL_BOOK,
                    "To Sell is Human",
                    "Daniel H. Pink",
                    html! {}
                ))
            }
            .reading-list {
                h2 { "Favorites" }
                (book(
                    UNDERSTANDING_BOOK,
                    "Understanding the Professional Programmer",
                    "Gerald M. Weinberg",
                    html! {
                        p {
                            "Although this book was written almost 40 years ago,
                            it was oddly comforting to see that the same struggles 
                            for programmers back then are still relevant today. 
                            Thinking about it again, maybe it's more disappointing than comforting. 
                            Nonetheless, I recommend it for programmers and managers of them." 
                        }
                    }
                ))
                (book(
                    NINETEEN84_BOOK,
                    "1984",
                    "George Orwell",
                    html! {
                        p {
                            "Does this one need an explanation?"
                        }
                    }
                ))
            }
        }
    }
}

fn book(cover: &str, title: &str, author: &str, thoughts: Markup) -> Markup {
    let book_cover = format!("/public/images/reading/{cover}");

    html! {
        .book {
            .book-cover {
                img src=(book_cover) alt=(title);
            }
            .book-info {
                h3 { (title) }
                p { b { "Author: " } (author) }
                (thoughts)
            }
        }
    }
}
