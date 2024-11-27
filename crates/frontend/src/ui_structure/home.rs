use crate::components::{IconButton, IconLink, IconizedItem};
use crate::styles::pane::PaneType::{Primary, Secondary};
use crate::styles::{pane, Css};
use std::convert::From;
use std::ops::Add;
use std::string::ToString;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    let main_css = css!(
        r#"
        background-size: cover;
        background-repeat: no-repeat;
        margin: 0 auto;
        padding-top: 100px;
        display: flex;
        justify-content: center;
        background-color: var(--color-tertiary-bkg-pane);
        
        @media (max-width: 1080px) {
          background-size: cover;
          background-repeat: no-repeat;
          margin: 0 auto;
          padding-top: 100px;
          display: flex;
          flex-wrap: wrap;
        }
        "#
    );

    let center_css = r#"
    width: 50%;
    
    iframe
    {
      border: none;
    }
    
    @media (max-width: 1080px) {
        :nth-child(2) {
          order: 3;
        }
        
        width: 100%;
        height: 910px;
    }
    "#
    .to_string()
    .add(&pane::PaneStyle::new(Primary).css())
    .into_css();

    let right_css = r#"
        width: fit-content;
        height: fit-content;
        
        @media (max-width: 700px) {
          display: flex;
          flex-direction: row;
          align-items: center;
        }
        "#
    .to_string()
    .add(&pane::PaneStyle::new(Primary).css())
    .into_css();

    let iframe_url = use_state(|| "static/html/about.html".to_string()); // Estado inicial de la URL del iframe

    let about_click = {
        let iframe_url = iframe_url.clone();
        Callback::from(move |_| iframe_url.set("static/html/about.html".to_string()))
    };

    let works_click = {
        let iframe_url = iframe_url.clone();
        Callback::from(move |_| iframe_url.set("static/html/works.html".to_string()))
    };

    html! {
        <main class={main_css}>
            <LeftPane />
            <div class={center_css}>
                <iframe src={(*iframe_url).clone()} width="100%" height="100%"></iframe>
            </div>
            <div class={right_css}>
                <IconButton icon_src="resources/img/icon/about.png" label="About" onclick={about_click}/>
                <IconButton icon_src="resources/img/icon/works.png" label="Works" onclick={works_click}/>
            </div>
        </main>
    }
}

#[function_component(LeftPane)]
fn left_pane() -> Html {
    let css = r#"
    
    width: fit-content;
    height: fit-content;
    display: flex;
    gap: 10px;
    flex-direction: column;
    align-items: center;
    
    #profile-img-container
    {
      height: 200px;
      width: 200px;
      align-content: center;
      overflow: hidden;
      border-radius: 10%;
      top: -100px;
    }

    h3
    {
      padding: 10px;
      font-size: 16px;
      font-weight: normal;
      margin: 5px;
      border-radius: 10px;
      background-color: var(--color-secondary-bkg-pane);
    }
    "#
    .to_string()
    .add(&pane::PaneStyle::new(Primary).css())
    .into_css();

    html! {
        <left-pane class={css}>
            <div id="profile-img-container">
                <img style="height: 277px; width: 200px;" id="profile-img" src="resources/img/profile.jpg" alt="Profile picture"/>
            </div>

            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2>{ "Borja Castellano" }</h2>
                <h3>{ "Desarrollador multiplataforma" }</h3>

                <div style="display: flex;">
                    <IconLink href="https://www.linkedin.com/in/borja-cas/" icon_src="resources/img/icon/linkedin.png" alt_text="LinkedIn" />
                    <IconLink href="https://www.instagram.com/_zocoo/" icon_src="resources/img/icon/instagram.png" alt_text="Instagram" />
                </div>
            </div>

            <ContactInfo />

            <DownloadCV />
        </left-pane>
    }
}

#[function_component(ContactInfo)]
fn contact_info() -> Html {
    let css_string = r#"
        display: flex;
        flex-direction: column;
        "#
    .to_string()
    .add(&pane::PaneStyle::new(Secondary).css());

    html! {
        <contact-info class={css_string.into_css()}>
            <IconizedItem icon_src="resources/img/icon/email.png" alt_text="Email" title="Email" detail="borjacastellano1@gmail.com" />
            <IconizedItem icon_src="resources/img/icon/phone.png" alt_text="Phone" title="Phone" detail="+34 681 240 207" />
            <IconizedItem icon_src="resources/img/icon/mapa.png" alt_text="Map" title="Ubicación" detail="Arcade 36690, Pontevedra" />
            <IconizedItem icon_src="resources/img/icon/reloj.png" alt_text="Clock" title="Disponibilidad horaria" detail="De tarde" />
            <IconizedItem icon_src="resources/img/icon/pasaporte.png" alt_text="Passport" title="Nacionalidad" detail="Española y Estadounidense" />
        </contact-info>
    }
}

#[styled_component(DownloadCV)]
fn download_cv() -> Html {
    let css = css!(
        r#"
        text-decoration: none;
        margin-top: 20px;
        border-radius: 10px;
        background-color: var(--color-button-hover);
        cursor: pointer;
        width: 220px;
        padding: 5px;
        display: flex;
        flex-direction: column;
        align-items: center;

        div
        {
          display: flex;
          align-items: center;
          gap: 5px;
        }

        img
        {
          height: 30px;
          width: 30px;
          filter: brightness(0) invert(1);
        }
        
        p
        {
          margin: 0;
          color: white;
          font-weight: bold;
        }
        "#
    );

    html! {
        <a class={css} href="static/docs/CV-Borja-Castellano-actual.pdf" target="_blank">
            <div>
                <img src="resources/img/icon/download.png" alt="Download" />
                <p>{ "Download CV" }</p>
            </div>
        </a>
    }
}
