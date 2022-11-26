use crate::{
    card::{Card, CardBody, CardFooter, CardHeader, CardText, CardTitle},
    form::{FormCheck, FormControl, FormControlType, FormGroup},
};
use yew::prelude::*;

pub mod card;
pub mod form;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let hide_card = true;
    let hide_button = true;
    let hide_check_box = true;
    let hide_color = true;
    let hide_date = true;
    let hide_data_time_local = true;
    let hide_email = true;
    let hide_file = false;
    let hide_password = true;

    html! {
        <div class={ "container" }>
            <Card hidden={ hide_card }>
                <CardHeader />
                <CardBody>
                    <CardTitle />
                    <CardText />
                </CardBody>
                <CardFooter />
            </Card>

            <form>
                <FormGroup hidden={ hide_button }>
                    <FormControl
                        id={ "exampleInputButton" }
                        input={ FormControlType::Button }
                        value={ "Button" } />
                </FormGroup>

                <FormCheck hidden={ hide_check_box }>
                    <FormControl
                        id={ "exampleInputCheckBoxEquine" }
                        input={ FormControlType::CheckBox }
                        name={ "Equine" }
                        value={ "Equus" } />
                    <FormControl
                        id={ "exampleInputCheckBoxUrsine" }
                        input={ FormControlType::CheckBox }
                        name={ "Ursine" }
                        value={ "Ursus" } />
                </FormCheck>

                <FormGroup hidden={ hide_color }>
                    <FormControl
                        id={ "exampleInputColor" }
                        input={ FormControlType::Color }
                        label={ "Color" }
                        name={ "Color" }
                        value={ "#ff0000" } />
                </FormGroup>

                <FormGroup hidden={ hide_date }>
                    <FormControl
                        id={ "exampleInputDate" }
                        input={ FormControlType::Date }
                        label={ "Date" }
                        name={ "Date" } />
                </FormGroup>

                <FormGroup hidden={ hide_data_time_local }>
                    <FormControl
                        id={ "exampleInputDateTimeLocal" }
                        input={ FormControlType::DateTimeLocal }
                        label={ "Date Time Local" }
                        name={ "DateTimeLocal" } />
                </FormGroup>

                <FormGroup hidden={ hide_email }>
                    <FormControl
                        id={ "exampleInputEmail1" }
                        input={ FormControlType::Email }
                        label={ "Email Address" }
                        placeholder={ "Enter Email" }
                        readonly=true />
                </FormGroup>

                <FormGroup hidden={ hide_file }>
                    <FormControl
                        id={ "exampleInputFile1" }
                        input={ FormControlType::File }
                        label={ "File" }
                        name={ "File" } />
                </FormGroup>

                <FormGroup hidden={ hide_password }>
                    <FormControl
                        disabled=true
                        id={ "exampleInputPassword1" }
                        input={ FormControlType::Password }
                        label={ "Password" }
                        placeholder={ "Password" } />
                </FormGroup>
            </form>
        </div>
    }
}
