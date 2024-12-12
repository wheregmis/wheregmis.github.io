use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct TestimonialProps {
    name: String,
    role: String,
    company: String,
    avatar: String,
    quote: String,
}

#[component]
pub fn Testimonials() -> Element {
    let testimonials = vec![
        TestimonialProps {
            name: "Sabin".into(),
            role: "I Myself".into(),
            company: "My Own World".into(),
            avatar: "https://api.uifaces.co/our-content/donated/xZ4wg2Xj.jpg".into(),
            quote: "I am fucking awesome and i know it, Before someput put testimonials i need to judge myself".into(),
        },
        TestimonialProps {
            name: "Kshitiz Bhattarai".into(),
            role: "My Friend".into(),
            company: "My Badass COmpany".into(),
            avatar: "https://api.uifaces.co/our-content/donated/xZ4wg2Xj.jpg".into(),
            quote: "Sabin is the coolest dev i ever know".into(),
        },
    ];

    rsx! {
        div { class: "container mx-auto px-4 py-12",
            // Section header
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2 text-center",
                "What People Say"
            }
            p { class: "text-gray-400 mb-12 text-center", "Testimonials from people I've worked with" }
            // Testimonials grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                for testimonial in testimonials {
                    div { class: "group relative overflow-hidden rounded-xl bg-gray-900/50 border border-gray-800 p-6 transition-all duration-300 hover:bg-gray-900/70 hover:border-gray-700",
                        // Quote
                        p { class: "text-gray-300 mb-6 italic",
                            ""
                            {testimonial.quote},
                            ""
                        }
                        // Author info
                        div { class: "flex items-center space-x-4",
                            img {
                                class: "w-12 h-12 rounded-full object-cover",
                                src: "{testimonial.avatar}",
                                alt: "{testimonial.name}"
                            }
                            div {
                                h4 { class: "text-white font-medium", "{testimonial.name}" }
                                p { class: "text-sm text-gray-400",
                                    "{testimonial.role} at {testimonial.company}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
