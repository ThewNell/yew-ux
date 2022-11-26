use crate::{
    card::{
        body::CardBody, card::Card, footer::CardFooter, header::CardHeader,
        text::CardText, title::CardTitle,
    },
    form::{
        check::FormCheck,
        control::{FormControl, FormControlType},
        group::FormGroup,
    },
};
use yew::prelude::*;

pub mod card;
pub mod form;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let show_card = false;

    let show_button = true;
    let show_check_box = false;
    let show_color = false;
    let show_date = true;
    let show_date_time_local = true;
    let show_email = true;
    let show_password = true;
    let show_form = show_button
        || show_check_box
        || show_color
        || show_email
        || show_password;

    html! {
        <div class={ "container" }>
            if show_card {
                <Card>
                    <CardHeader />
                    <CardBody>
                        <CardTitle />
                        <CardText />
                    </CardBody>
                    <CardFooter />
                </Card>
            }

            if show_form {
                <form>
                    if show_button {
                        <FormGroup>
                            <FormControl id={ "exampleInputButton" } input={ FormControlType::Button }
                                value={ "Button" }>
                            </FormControl>
                        </FormGroup>
                    }

                    if show_check_box {
                        <FormCheck>
                            <FormControl id={ "exampleInputCheckBoxEquine" } input={ FormControlType::CheckBox }
                                name={ "Equine" }
                                value={ "Equus" }>
                            </FormControl>
                            <FormControl id={ "exampleInputCheckBoxUrsine" } input={ FormControlType::CheckBox }
                                name={ "Ursine" }
                                value={ "Ursus" }>
                            </FormControl>
                        </FormCheck>
                    }

                    if show_color {
                        <FormGroup>
                            <FormControl id={ "exampleInputColor" } input={ FormControlType::Color }
                                label={ "Color" }
                                name={ "Color" }
                                value={ "#ff0000" }>
                            </FormControl>
                        </FormGroup>
                    }

                    if show_date {
                        <FormGroup>
                            <FormControl id={ "exampleInputDate" } input={ FormControlType::Date }
                                label={ "Date" }
                                name={ "Date" }>
                            </FormControl>
                        </FormGroup>
                    }

                    if show_date_time_local {
                        <FormGroup>
                            <FormControl id={ "exampleInputDateTimeLocal" } input={ FormControlType::DateTimeLocal }
                                label={ "Date Time Local" }
                                name={ "DateTimeLocal" }>
                            </FormControl>
                        </FormGroup>
                    }

                    if show_email {
                        <FormGroup>
                            <FormControl id={ "exampleInputEmail1" } input={ FormControlType::Email }
                                label={ "Email Address" }
                                placeholder={ "Enter Email" }>
                            </FormControl>
                        </FormGroup>
                    }

                    if show_password {
                        <FormGroup>
                            <FormControl id={ "exampleInputPassword1" } input={ FormControlType::Password }
                                label={ "Password" }
                                placeholder={ "Password" }>
                            </FormControl>
                        </FormGroup>
                    }
                </form>
            }
        </div>
    }
}
