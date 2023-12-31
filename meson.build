project('prisma-timer', 'rust', 
          version: '0.1.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

app_id = 'io.github.manenfu.PrismaTimer'
pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()

i18n = import('i18n')
gnome = import('gnome')



subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
