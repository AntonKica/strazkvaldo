import type { PageLoad } from './$types';
import { SVC_ENUM_APP_USER_ROLE, SVC_ENUM_CRITICALITY_TYPE, SVC_ENUM_ROOM_TYPE, SVC_ENUM_ACTIVITY_TYPE } from '$lib/serviceRoutes'


export const load: PageLoad = async ({ params }) => {
    let app_user_role = await fetch(SVC_ENUM_APP_USER_ROLE.LIST()).then(response => {
        return response.json();
    });
    let criticality_type = await fetch(SVC_ENUM_CRITICALITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let room_type = await fetch(SVC_ENUM_ROOM_TYPE.LIST()).then(response => {
        return response.json();
    });
    let activity_type = await fetch(SVC_ENUM_ACTIVITY_TYPE.LIST()).then(response => {
        return response.json();
    });

	return {
		app_user_role: app_user_role.enum_values,
		criticality_type: criticality_type.enum_values,
		room_type: room_type.enum_values,
		activity_type: activity_type.enum_values,
	};
}