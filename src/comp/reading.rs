use maud::{Markup, html};

const BLINK_BOOK: &str = "/public/images/reading/blink.jpg";
const HOUSE_BOOK: &str = "/public/images/reading/house.jpg";
const TOSELL_BOOK: &str = "/public/images/reading/tosell.jpg";
const UNDERSTANDING_BOOK: &str = "/public/images/reading/understandingprogrammer.jpg";

pub fn reading() -> Markup {
    html! {
        .body-container {
            .reading-list {
                h2 { "Currently Reading" }
                (book(
                    BLINK_BOOK,
                    "Blink",
                    "Malcolm Gladwell",
                    html! {
                        p {
                            "It has been very enjoyable so far with lots of fun anecdotes to 
                            compliment the author's points."
                        }
                    }
                ))
                (book(
                    HOUSE_BOOK,
                    "House of Leaves",
                    "Mark Z. Danielewski",
                    html! {
                        p {
                            "I, like many other readers, was inspired to start reading this book 
                            after watching the myhouse.wad video essay. 
                            I really enjoy the rich vocabulary and unique narrative structure. 
                            Anytime I sit myself down to continue reading, 
                            I find myself bringing a dictionary with me just in case :)"
                        }
                    }
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
                            Nonetheless, It's a must read for programmers and managers at any level." 
                        }
                    }
                ))
            }
        }
    }
}

fn book(cover: &str, title: &str, author: &str, thoughts: Markup) -> Markup {
    html! {
        .book {
            .book-cover {
                img src=(cover) alt=(title);
            }
            .book-info {
                h3 { (title) }
                p { b { "Author: " } (author) }
                (thoughts)
            }
        }
    }
}