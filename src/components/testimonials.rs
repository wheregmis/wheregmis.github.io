use dioxus::hooks::*;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

use crate::PROFILE_PIC;

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
            avatar: PROFILE_PIC.to_string(),
            quote: "I am fucking awesome and i know it, and i dont need someone else to judge me"
                .into(),
        },
        TestimonialProps {
            name: "Kshitiz Bhattarai".into(),
            role: "My Friend".into(),
            company: "My Badass COmpany".into(),
            avatar: "https://api.uifaces.co/our-content/donated/xZ4wg2Xj.jpg".into(),
            quote: "Sabin is the coolest dev i have ever known".into(),
        },
    ];

    let mut container_transform = use_motion(Transform::new(0.0, 20.0, 0.95, 0.0));
    let mut container_opacity = use_motion(0.0f32);

    use_effect(move || {
        container_transform.animate_to(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                ..Default::default()
            })),
        );

        container_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(800),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );
    });

    rsx! {
        div {
            class: "container mx-auto px-4 py-12",
            style: "transform: translateY({container_transform.get_value().y}px) scale({container_transform.get_value().scale}); opacity: {container_opacity.get_value()};",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500 mb-2 text-center",
                "What People Say"
            }
            p { class: "text-gray-400 mb-12 text-center", "Testimonials from people I've worked with" }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                {
                    testimonials
                        .iter()
                        .enumerate()
                        .map(|(index, testimonial)| {
                            let mut card_transform = use_motion(
                                Transform::new(-20.0, 0.0, 0.95, 0.0),
                            );
                            use_effect(move || {
                                let delay = Duration::from_millis(300 + index as u64 * 200);
                                card_transform
                                    .animate_to(
                                        Transform::identity(),
                                        AnimationConfig::new(
                                                AnimationMode::Spring(Spring {
                                                    stiffness: 100.0,
                                                    damping: 15.0,
                                                    mass: 1.0,
                                                    ..Default::default()
                                                }),
                                            )
                                            .with_delay(delay),
                                    );
                            });
                            rsx! {
                                div {
                                    class: "group relative overflow-hidden rounded-xl bg-surface/50 border border-surface-light/20 p-6 transition-all duration-300 hover:bg-surface-hover hover:border-surface-light/40",
                                    style: "transform: translateX({card_transform.get_value().x}px) scale({card_transform.get_value().scale});",


                                    p { class: "text-text-secondary mb-6 italic",
                                        ""
                                        "{testimonial.quote}"
                                        ""
                                    }

                                    div { class: "flex items-center space-x-4 transition-transform duration-300 group-hover:translate-y-[-2px]",
                                        img {
                                            class: "w-12 h-12 rounded-full object-cover transition-transform duration-300 group-hover:scale-110",
                                            src: "{testimonial.avatar}",
                                            alt: "{testimonial.name}",
                                        }
                                        div {
                                            h4 { class: "text-text-primary font-medium", "{testimonial.name}" }
                                            p { class: "text-sm text-text-muted", "{testimonial.role} at {testimonial.company}" }
                                        }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
