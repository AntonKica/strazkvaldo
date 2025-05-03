import type { PageLoad } from './$types';
import { goto } from '$app/navigation';
import { SVC_ENUM_GENERIC } from '$lib/serviceRoutes'
import { UI_ADMIN_ENUM } from '$lib/uiRoutes';


export const load: PageLoad = async ({ fetch, params }) => {
    let enum_result = await fetch(SVC_ENUM_GENERIC(params.name));
    if (!enum_result.ok) {
        alert("Neznámy typ číselníka");
		goto(UI_ADMIN_ENUM, { invalidateAll: true});
    }
    let enum_json = await enum_result.json();
    
    let enum_name = "";
    switch(params.name) {
        case "room-type": {
            enum_name = "typ miestnosti"
            break;
        };
        case "activity-type": {
            enum_name = "typ aktivity"
            break;
        };
    }

	return {
        ...enum_json,
        name: params.name,
        enum_name: enum_name
	};
}