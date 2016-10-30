// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod create {
    use std::fs::create_dir_all;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::collections::HashMap;

    use handlebars::Handlebars;

    use common::ui::{UI, Status};
    use error::Result;

    const PLAN_TEMPLATE: &'static str = "plan {{pkg_origin}}/{{pkg_name}}";
    const RUN_HOOK_TEMPLATE: &'static str = "hook {{pkg_origin}}/{{pkg_name}}";

    pub fn start(ui: &mut UI, origin: String, name: String) -> Result<()> {
        try!(ui.begin("Constructing a cozy habitat for your app..."));
        try!(ui.br());

        // Build out the variables passed.
        let handlebars = Handlebars::new();
        let mut data = HashMap::new();
        data.insert("pkg_name".to_string(), name);
        data.insert("pkg_origin".to_string(), origin);

        let rendered_plan = try!(handlebars.template_render(PLAN_TEMPLATE, &data));
        try!(create_with_template(ui, "habitat/plan.sh", rendered_plan));
        try!(ui.para("The `plan.sh` is the foundation of your new habitat. You can \
            define core metadata, dependencies, and tasks. More documentation here: TODO"));

        let rendered_run_hook = try!(handlebars.template_render(RUN_HOOK_TEMPLATE, &data));
        try!(create_with_template(ui, "habitat/hooks/run", rendered_run_hook));
        try!(ui.para("The `hooks` directory is where you can create a number of automation hooks \
            into your habitat. We'll make a `run` hook to get you started, but there are more \
            hooks to create and tweak! See the full list with info here: TODO"));

        try!(ui.end("A happy abode for your code has been initialized! Now it's time to explore!"));
        Ok(())
    }

    fn create_with_template(ui: &mut UI, location: &str, template:  String) -> Result<()> {
        let path = Path::new(&location);
        match path.exists() {
            false => {
                try!(ui.status(Status::Creating, format!("file: {}", location)));
                // If the directory doesn't exist we need to make it.
                if let Some(directory) = path.parent() {
                    try!(create_dir_all(directory));
                }
                // Create and then render the template with Handlebars
                try!(File::create(path).and_then(|mut file| file.write(template.as_bytes())));
            },
            true => {
                // If the user has already configured a file overwriting would be impolite.
                try!(ui.status(Status::Using, format!("existing file: {}", location)));
            }
        };
        Ok(())
    }
}
