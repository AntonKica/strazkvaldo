<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
	import { goto } from '$app/navigation';
    import { SVC_USER_RECENTLY_FINISHED_ACTIVITY } from '$lib/serviceRoutes';
    import { UI_USER_RECENTLY_FINISHED_ACTIVITY } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
	const recently_finished_activity = data.recently_finished_activity;
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();
		

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);

        const formatted_data = {
			...formEntries,
        };
		
		const res = await fetch(SVC_USER_RECENTLY_FINISHED_ACTIVITY.CUSTOM_CODE("review",recently_finished_activity.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to review")
			return;			
		}

		alert("Sucessfully revied");
		goto(UI_USER_RECENTLY_FINISHED_ACTIVITY.LIST(), { invalidateAll: true});
	}
	
</script>

<form method="POST" onsubmit={handleSubmit}>
	<b>kód</b> {recently_finished_activity.code} <br>
	<b>dátum</b> {recently_finished_activity.due_date} <br>
	<label>popis<textarea name="description">{recently_finished_activity.description}</textarea></label> <br><br>
	<button type="submit">Uzavri</button>
</form>