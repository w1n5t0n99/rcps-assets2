use crate::{router::Route, store::Store};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, _) = use_store::<Store>();
    let user = store.auth_user.clone();

    html! {
        <header class="bg-white h-20">
        <nav class="h-full flex justify-between container items-center">
          <div>
            <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"CodevoWeb"}</Link<Route>>
          </div>
          <ul class="flex items-center gap-4">
            <li>
              <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"Home"}</Link<Route>>
            </li>
            if user.is_some() {
               <>
                <li>
                  <Link<Route> to={Route::ProfilePage} classes="text-ct-dark-600">{"Profile"}</Link<Route>>
                </li>
                <li
                  class="cursor-pointer"
                >
                  {"Create Post"}
                </li>
                <li class="cursor-pointer">
                  {"Logout"}
                </li>
              </>

            } else {
              <>
                <li>
                  <Link<Route> to={Route::RegisterPage} classes="text-ct-dark-600">{"SignUp"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::LoginPage} classes="text-ct-dark-600">{"Login"}</Link<Route>>
                </li>
              </>
            }
          </ul>
        </nav>
      </header>
    }
}
