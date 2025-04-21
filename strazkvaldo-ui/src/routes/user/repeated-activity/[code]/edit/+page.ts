import { SVC_USER_REPEATED_ACTIVITY, SVC_ENUM_ACTIVITY_TYPE, SVC_ENUM_CRITICALITY_TYPE, SVC_ENUM_PERIODICITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let repeated_activity = await fetch(SVC_USER_REPEATED_ACTIVITY.GET(params.code)).then(response => {
        return response.json();
    });
    let activity_type_result = await fetch(SVC_ENUM_ACTIVITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let criticality_type_result = await fetch(SVC_ENUM_CRITICALITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let periodicity_result = await fetch(SVC_ENUM_PERIODICITY.LIST()).then(response => {
        return response.json();
    });
    return {
        repeated_activity: repeated_activity.data.repeated_activity,
        activity_types: activity_type_result.enum_values,
        criticality_types: criticality_type_result.enum_values,
        periodicity: periodicity_result.enum_values,
    };
};