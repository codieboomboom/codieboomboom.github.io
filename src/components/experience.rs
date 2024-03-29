use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Referrer {
    pub name: String,
    pub title: String,
    pub contact: String,
}

#[derive(Clone, PartialEq)]
pub struct JobInfo {
    pub company: String,
    pub position: String,
    pub time: String,
    pub techstack: String,
    pub responsibility: Vec<&'static str>,
    pub referrer: Option<Referrer>,
}

#[derive(Properties, PartialEq)]
pub struct JobInfoProps {
    info: JobInfo,
}

#[function_component(Job)]
pub fn job(props: &JobInfoProps) -> Html {
    let job_resp_details: Html = props
        .info
        .responsibility
        .iter()
        .map(|line| {
            html! {
                <li>{line}</li>
            }
        })
        .collect();
    
    let referrer_details: Html;
    if let Some(person) = &props.info.referrer {
        referrer_details = html! (
            <p>{"Referrer: "}<a href={person.contact.clone()}>{format!("{} - {}", person.name, person.title)}</a></p>
        )
    } else {
        referrer_details = html! ()
    }

    html!(
        <div class="experience-card">
            <h2>{format!("{} - {}",props.info.company, props.info.position)}</h2>
            <h3>{props.info.time.clone()}</h3>
            <u>{job_resp_details}</u>
            <p class="techstack">{format!("Technologies used: {}",props.info.techstack)}</p>
            {referrer_details}
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ExperienceListProps {
    pub jobs: Vec<JobInfo>,
}

#[function_component(ExperienceList)]
pub fn experience_list(ExperienceListProps { jobs }: &ExperienceListProps) -> Html {
    html! {
        <div class="experience-section">
        <h1>{"Experience"}</h1>
        {jobs.iter().map(|job| html! {
            <Job info={job.clone()}/>
        }).collect::<Html>()}
        </div>
    }
}
