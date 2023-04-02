use bounce::helmet::{Helmet, HelmetBridge};
use bounce::BounceRoot;
use gravatar_rs::Generator;
use yew::prelude::*;

const EMAIL: &str = "exilvm@disroot.org";

fn get_profile_avatar() -> String {
    let generator = Generator::default().set_image_size(600);
    generator.generate(EMAIL)
}

#[function_component(AboutMe)]
fn about_me() -> Html {
    let second_bio = use_state(|| false);

    let onclick = {
        let second_bio = second_bio.clone();
        Callback::from(move |_| second_bio.set(!*second_bio))
    };

    let second_bio_content = if *second_bio {
        html! {
            <div style="font-weight: 300;">
                <p>
                    <small>{ " (・_・ヾ)" }</small>
                </p>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <section>
            <h1>
                <a
                    style="text-transform: uppercase; color: #07d404; text-decoration: none;"
                    target="_blank"
                    title="The source code of this page"
                    href="https://github.com/exilvm/exilvm/tree/site"
                    rel="noreferrer"
                >
                    { "3x!1" }
                </a>
            </h1>
            <img
                alt="Avatar"
                {onclick}
                style="margin-top: 1rem; margin-bottom: 1.7rem; border-radius: 50%; width: 150px; cursor: pointer;"
                src={get_profile_avatar().clone()}
            />
            <h4>{ "exiluuuuuuu" }</h4>
            { second_bio_content }
            <div style="padding: 0 38px;">
                <p>{ "__software_dev__" }</p>
                <p>{ "__sysadmin__" }</p>
                <p>{ "__ui/ux_design__" }</p>
            </div>
        </section>
    }
}

#[derive(Clone, PartialEq)]
struct ListItem {
    children: &'static str,
    href: &'static str,
    target: Option<&'static str>,
}

#[function_component(Socials)]
fn socials() -> Html {
    let list = vec![
        ListItem {
            children: "Twitter",
            href: "https://twitter.com/exilvm",
            target: Some("_blank"),
        },
        ListItem {
            children: "Telegram",
            href: "tg://resolve?domain=exilvm",
            target: None,
        },
        ListItem {
            children: "Github",
            href: "https://github.com/exilvm",
            target: Some("_blank"),
        },
        ListItem {
            children: "Steam",
            href: "https://steamcommunity.com/id/exilvm",
            target: Some("_blank"),
        },
        ListItem {
            children: "AniList",
            href: "https://anilist.co/user/exilvm",
            target: Some("_blank"),
        },
        ListItem {
            children: "Last.fm",
            href: "https://last.fm/user/exilvm",
            target: Some("_blank"),
        },
    ];

    html! {
        <section>
            <h4>{ "My @'s" }</h4>
            <ul>
                { for list.iter().enumerate().map(|(i, item)| html! {
                    <li key={i}>
                        <p>
                            <a href={item.href} target={item.target.clone().unwrap_or("")}>{ item.children }</a>
                        </p>
                    </li>
                })}
            </ul>
        </section>
    }
}

#[function_component(Contact)]
fn contact() -> Html {
    let aliased_email = "exilvm+9x01@disroot.org";
    let mailto = format!("mailto:{}", aliased_email);

    html! {
        <section>
            <h4>{ "Contact" }</h4>
            <p>
                { "Email: " }
                <a href={mailto}>{ aliased_email }</a>
            </p>
        </section>
    }
}

#[derive(Clone, PartialEq)]
struct CryptoItem {
    name: &'static str,
    address: Html,
}

#[function_component(Crypto)]
fn crypto() -> Html {
    let list = vec![
        CryptoItem {
            name: "BTC",
            address: html! { "17nFjdytthofdha6aSmBDSHnb9NcSAhd8Z" },
        },
        CryptoItem {
            name: "BTC (Bech32)",
            address: html! { "bc1q0rhwyzc8upxjzn0qddz2rhv86xh3qvsydjn865" },
        },
        CryptoItem {
            name: "ETH",
            address: html! { "0x677C5728D3E26152187Cc98E06CB47A58EF58eaA" },
        },
        CryptoItem {
            name: "XMR",
            address: html! {
                "46SXLx7PGvm4u7FnDvNvoPYu48AP4fUfbHwCDw6koxbhUrT1bSYvuHe7et7q8uSJ52NJApxXUyLmX7AaJP7ysXcLRyeZaaz"
            },
        },
        CryptoItem {
            name: "PGP key",
            address: html! {
                <a target="_blank" rel="noreferrer" href="https://github.com/exilvm/exilvm/blob/master/key.pgp">{ "96120B10AD05294F" }</a>
            },
        },
    ];

    html! {
        <section>
            <h4>{ "Cryptos" }</h4>
            <ul>
                { for list.iter().enumerate().map(|(i, item)| html! {
                    <li key={i}>
                        <p>{ item.name }{ ":" }</p>
                        <pre>{ item.address.clone() }</pre>
                    </li>
                })}
            </ul>
        </section>
    }
}

#[derive(Clone, PartialEq)]
struct ResourceItem {
    children: &'static str,
    href: &'static str,
    target: &'static str,
}

#[function_component(Resources)]
fn resources() -> Html {
    let list = vec![
        ResourceItem {
            children: "InstallGentoo Wiki",
            href: "https://wiki.installgentoo.com/wiki/Main_Page",
            target: "_blank",
        },
        ResourceItem {
            children: "/g/",
            href: "https://boards.4channel.org/g/#catalog",
            target: "_blank",
        },
        ResourceItem {
            children: "LAINCHAN",
            href: "https://lainchan.org",
            target: "_blank",
        },
        ResourceItem {
            children: "PrivacyGuides",
            href: "https://www.privacyguides.org",
            target: "_blank",
        },
        ResourceItem {
            children: "HackerNews",
            href: "https://news.ycombinator.com",
            target: "_blank",
        },
    ];

    html! {
        <section>
            <h4>{ "Good resources" }</h4>
            <ul>
                { for list.iter().enumerate().map(|(i, item)| html! {
                    <li key={i}>
                        <p>
                            <a href={item.href} target={item.target}>{ item.children }</a>
                        </p>
                    </li>
                })}
            </ul>
        </section>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BounceRoot>
        <HelmetBridge default_title="default title" />
        <Helmet>
        <link rel="icon" href={get_profile_avatar().clone()} />
        <link rel="apple-touch-icon" href={get_profile_avatar().clone()} />
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="theme-color" content="#000000" />
        <meta name="description" content="exilvm - Personal page" />
        <title>{"./exilvmlinuz"}</title>
        <link rel="icon" href={get_profile_avatar().clone()} />
        <link rel="apple-touch-icon" href={get_profile_avatar().clone()} />
        <meta name="manifest" content="{
            &quot;name&quot;: &quot;./exilvmlinuz&quot;,
            &quot;start_url&quot;: &quot;.&quot;,
            &quot;display&quot;: &quot;fullscreen&quot;,
            &quot;theme_color&quot;: &quot;#efff00&quot;,
            &quot;background_color&quot;: &quot;#000000&quot;
            }" />
        </Helmet>
        </BounceRoot>
        <div style="width: 390px">
        <AboutMe />
        <Socials />
        <Contact />
        <Crypto />
        <Resources />
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
