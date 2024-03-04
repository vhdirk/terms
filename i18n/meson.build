#i18n.gettext(
#	'done',
#	args: [
#	    '--keyword=i18n'
#	  ],
#	preset: 'glib'
#)

add_project_arguments('-DGETTEXT_PACKAGE="terms"', language:'rust')
i18n.gettext(gettext_package,
             args: ['--keyword=gettext', '--keyword=ngettext:1,2',],
             preset: 'glib')
