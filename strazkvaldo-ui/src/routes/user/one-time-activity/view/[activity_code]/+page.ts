import type { pageload } from './$types';

export const load: pageload = async ({ params, parent }) => {
    let { one_time_activities } = await parent() ?? {};
    console.log(params.activity_code)
        return (one_time_activities ?? []).find((a) => a.code === params.activity_code);
};