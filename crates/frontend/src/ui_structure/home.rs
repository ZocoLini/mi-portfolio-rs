use crate::components::{IconButton, IconLink, IconizedItem};
use crate::styles::pane::PaneType::{Primary, Secondary};
use crate::styles::{pane, Css};
use std::ops::Add;
use std::string::ToString;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    let css = css!(
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
    
    html! {
        <main class={css}>
            <LeftPane />
            <CenterPane />
            <RightPane />
        </main>
    }
}

// region: Left Pane

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
                <img style="height: 277px; width: 200px;" id="profile-img" src="static/img/profile.jpg" alt="Profile picture"/>
            </div>

            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2>{ "Borja Castellano" }</h2>
                <h3>{ "Desarrollador multiplataforma" }</h3>

                <div style="display: flex;">
                    <IconLink href="https://www.linkedin.com/in/borja-cas/" icon_src="static/img/icon/linkedin.png" alt_text="LinkedIn" />
                    <IconLink href="https://www.instagram.com/_zocoo/" icon_src="static/img/icon/instagram.png" alt_text="Instagram" />
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
            <IconizedItem icon_src="static/img/icon/email.png" alt_text="Email" title="Email" detail="borjacastellano1@gmail.com" />
            <IconizedItem icon_src="static/img/icon/phone.png" alt_text="Phone" title="Phone" detail="+34 681 240 207" />
            <IconizedItem icon_src="static/img/icon/mapa.png" alt_text="Map" title="Ubicación" detail="Arcade 36690, Pontevedra" />
            <IconizedItem icon_src="static/img/icon/reloj.png" alt_text="Clock" title="Disponibilidad horaria" detail="De tarde" />
            <IconizedItem icon_src="static/img/icon/pasaporte.png" alt_text="Passport" title="Nacionalidad" detail="Española y Estadounidense" />
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
                <img src="static/img/icon/download.png" alt="Download" />
                <p>{ "Download CV" }</p>
            </div>
        </a>
    }
}

// endregion

// region: Center Pane

#[function_component(CenterPane)]
fn center_pane() -> Html {
    let css = r#"
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
    "#.to_string().add(&pane::PaneStyle::new(Primary).css()).into_css();
    
    html!(
        <div class={css}>
            <iframe src="static/html/about.html" width="100%" height="100%"></iframe>
        </div>
    )
}

// endregion

// region: Right Pane

#[function_component(RightPane)]
fn right_pane() -> Html {
    let css = r#"
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

    html!(
        <div class={css}>
            <IconButton icon_src="static/img/icon/about.png" label="About" />
            <IconButton icon_src="static/img/icon/works.png" label="Works" />
        </div>
    )
}

// endregion
