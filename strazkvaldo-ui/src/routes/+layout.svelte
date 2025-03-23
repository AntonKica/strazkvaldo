<script>
    import { base } from '$app/paths';
	let { children } = $props();
</script>

<style>
	ul#menu li{
		display : inline;
	}
</style>


<ul id="menu">
<!--
 usecase "prihlásiť/odhlásiť sa" as ucLogin
    usecase "zaregistrovať sa" as ucSignin

    usecase "správa používateľov" as ucUserManagement
    usecase "správa číselníkov" as ucEnumManagement

    usecase "správa miestností" as ucRoomManagement
    usecase "správa aktivít" as ucActivityManagement
    usecase "prehľad najbližších aktivít" as ucUpcomingActivityList
    usecase "vyhodnoť nedávne aktivíty" as ucUpcomingActivityList
    usecase "prehľad všetkých aktivít" as ucActivityList
    usecase "prehľad vykonaných aktivít" as ucFinishedActivityList
    
    usecase "nastavenie upozornení" as ucNotificationSettings
	-->
	<li><a href="{base}/">Domov</a></li> |
	<li><a href="{base}/about">O aplikácií</a></li> |
	|
	<li style="color:indianred">Admin</li> |
	<li><a style="color:indianred" href="{base}/admin/user">Správa používateľov</a></li> |
	<li><a style="color:indianred" href="{base}/admin/enum">Správa číselníkov</a></li> |
	|
	<li style="color:#117a65">Užívateľ</li> |
	<li><a style="color:#117a65" href="{base}/user/rooms">Správa miestnosti</a></li> |
	<li><a style="color:#117a65" href="{base}/user/activity">Správa aktivít</a></li> |
	<li><a style="color:#117a65" href="{base}/user/upcoming-activities">Najbližšie aktivity</a></li> |
	<li><a style="color:#117a65" href="{base}/user/recent-activities">Nedávne aktivity</a></li> |
	<li><a style="color:#117a65" href="{base}/user/notifications">Notifikácie</a></li> |
	<li><a style="color:#117a65" href="{base}/user/settings">Nastavenia</a></li> |
</ul>

{@render children()}