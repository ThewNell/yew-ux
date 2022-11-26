use crate::{
    card::{Card, CardBody, CardFooter, CardHeader, CardText, CardTitle},
    form::{FormCheck, FormClass, FormControl, FormControlType, FormGroup},
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
    let hide_button = false;
    let hide_check_box = true;
    let hide_color = false;
    let hide_date = false;
    let hide_data_time_local = false;
    let hide_email = false;
    let hide_file = false;
    let hide_hidden = false;
    let hide_month = false;
    let hide_number = false;
    let hide_password = false;
    let hide_range = false;
    let hide_reset = false;

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
                        readonly=false />
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
                        disabled=false
                        id={ "exampleInputPassword1" }
                        input={ FormControlType::Password }
                        label={ "Password" }
                        placeholder={ "Password" } />
                </FormGroup>

                <FormGroup hidden={ hide_hidden }>
                    // TODO look into height/width overriding form-control styling
                    <FormControl
                        alternate={ "insectoid-male-1" }
                        id={ "exampleInputImage" }
                        input={ FormControlType::Image }
                        source={ "insectoid_male_01_noiseless.png" }
                        height=128
                        width=128
                        title={ "Not much is known about the Insectoid race.".to_owned() } />
                </FormGroup>

                <FormGroup hidden={ hide_month }>
                    <FormControl
                        id={ "exampleInputMonth" }
                        input={ FormControlType::Month }
                        name={ "renewalMonth" } />
                </FormGroup>

                <FormGroup hidden={ hide_number }>
                    <FormControl
                        id={ "exampleInputNumber" }
                        input={ FormControlType::Number }
                        name={ "someNumber" }
                        max=30_f32
                        min=1_f32 />
                </FormGroup>

                <FormGroup hidden={ hide_range }>
                    <FormControl
                        id={ "exampleInputRange" }
                        input={ FormControlType::Range }
                        label={ "Some Range"}
                        name={ "someRange" }
                        max=10_f32
                        min=1_f32
                        step=0.5
                        value={ "5.0" } />
                </FormGroup>

                <FormGroup hidden={ hide_reset }>
                    <FormControl
                        id={ "exampleInputReset" }
                        input={ FormControlType::Reset }
                        value={ "Clear Form" } />
                </FormGroup>
            </form>
        </div>
    }
}
