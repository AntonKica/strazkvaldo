<script lang="ts">
	import type { PageProps } from './$types';
	import { invalidateAll } from '$app/navigation';
    import { SVC_USER_APP_SETTINGS } from '$lib/serviceRoutes';

	let { data }: PageProps = $props();
	const app_settings = data.app_settings;
	let auto_review_finished_activity = $state(app_settings.auto_review_finished_activity);
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();
		
        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);

        console.log(formEntries.auto_review_finished_activity);
        const formatted_data = {
            auto_review_finished_activity: formEntries.auto_review_finished_activity === "true",
            auto_review_finished_activity_days: Number(formEntries.auto_review_finished_activity_days ?? 0),
        };
		
		const res = await fetch(SVC_USER_APP_SETTINGS, {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to update settings")
			return;			
		}

		alert("Sucessfully update settings");
	}
	
</script>

<form method="POST" onsubmit={handleSubmit}>
	<h4>Nastavenie automatického uzatvárania dokončených aktivít</h4>
	<label>zapnúť <input type="checkbox" name="auto_review_finished_activity" bind:checked={auto_review_finished_activity} value={auto_review_finished_activity}/></label> <br>
	<label>uzavrieť po <input type="number" min="0" name="auto_review_finished_activity_days" value={app_settings.auto_review_finished_activity_days} disabled={auto_review_finished_activity === false}/> dňoch</label> <br>
    <br>
	<button type="submit">Ulož</button>
</form>