use yew::prelude::*;
use crate::components::IconButton;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <main>
            <Profile />
            <div id="center-panel" class="pane item">
                <iframe src="static/html/about.html" width="100%" height="100%"></iframe>
            </div>
            <div id="right-pane" class="item pane fit-content">
                <IconButton icon_src="static/img/icon/about.png" label="About" />
                <IconButton icon_src="static/img/icon/works.png" label="Works" />
            </div>
        </main>
    }
}

#[function_component(Profile)]
fn profile() -> Html {
    html! {
        <div id="left-panel" class="item pane vertical-flex centered-flex fit-content">
            <div id="profileImg-container">
                <img id="profileImg" src="static/img/profile.jpg" alt="Profile picture"/>
            </div>

            <div id="presentacion" class="vertical-flex centered-flex">
                <h2>{ "Borja Castellano" }</h2>
                <div class="iBox primary-text">{ "Desarrollador multiplataforma" }</div>

                <div id="socialMedia" class="flex">
                    <SocialIcon href="https://www.linkedin.com/in/borja-cas/" icon_src="static/img/icon/linkedin.png" alt="LinkedIn" />
                    <SocialIcon href="https://www.instagram.com/_zocoo/" icon_src="static/img/icon/instagram.png" alt="Instagram" />
                </div>
            </div>

            <ContactInfo />

            <a id="dowloadCV" class="vertical-flex centered-flex" href="static/docs/CV-Borja-Castellano-actual.pdf" target="_blank">
                <div class="flex centered-flex">
                    <img class="icon" src="static/img/icon/download.png" alt="Download" />
                    <p>{ "Download CV" }</p>
                </div>
            </a>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SocialIconProps {
    pub href: String,
    pub icon_src: String,
    pub alt: String,
}

#[function_component(SocialIcon)]
fn social_icon(props: &SocialIconProps) -> Html {
    html! {
        <div class="iBox">
            <a href={props.href.clone()} target="_blank">
                <img class="icon" src={props.icon_src.clone()} alt={props.alt.clone()} />
            </a>
        </div>
    }
}

#[function_component(ContactInfo)]
fn contact_info() -> Html {
    html! {
        <div id="contactInfo" class="vertical-flex iBox">
            <ContactItem icon_src="static/img/icon/email.png" title="Email" detail="borjacastellano1@gmail.com" />
            <ContactItem icon_src="static/img/icon/phone.png" title="Phone" detail="+34 681 240 207" />
            <ContactItem icon_src="static/img/icon/mapa.png" title="Ubicación" detail="Arcade 36690, Pontevedra" />
            <ContactItem icon_src="static/img/icon/reloj.png" title="Disponibilidad horaria" detail="De tarde" />
            <ContactItem icon_src="static/img/icon/pasaporte.png" title="Nacionalidad" detail="Española y Estadounidense" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContactItemProps {
    pub icon_src: String,
    pub title: String,
    pub detail: String,
}

#[function_component(ContactItem)]
fn contact_item(props: &ContactItemProps) -> Html {
    html! {
        <div class="contactItem">
            <img class="icon" src={props.icon_src.clone()} alt={props.title.clone()} />
            <div class="vertical-flex">
                <p class="third-text">{ &props.title }</p>
                <p class="second-text">{ &props.detail }</p>
            </div>
        </div>
    }
}