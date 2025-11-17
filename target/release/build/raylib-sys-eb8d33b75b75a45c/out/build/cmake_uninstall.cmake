if(NOT EXISTS "/mnt/c/Users/isabe/OneDrive/Escritorio/FINAL_GRAFICAS/Graficas_ENTREGAS/GraficasLab4/lab4/target/release/build/raylib-sys-eb8d33b75b75a45c/out/build/install_manifest.txt")
  message(FATAL_ERROR "Cannot find install manifest: /mnt/c/Users/isabe/OneDrive/Escritorio/FINAL_GRAFICAS/Graficas_ENTREGAS/GraficasLab4/lab4/target/release/build/raylib-sys-eb8d33b75b75a45c/out/build/install_manifest.txt")
endif()

file(READ "/mnt/c/Users/isabe/OneDrive/Escritorio/FINAL_GRAFICAS/Graficas_ENTREGAS/GraficasLab4/lab4/target/release/build/raylib-sys-eb8d33b75b75a45c/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach(file ${files})
  message(STATUS "Uninstalling $ENV{DESTDIR}${file}")
  if(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    exec_program(
      "/usr/bin/cmake" ARGS "-E remove \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
      )
    if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing $ENV{DESTDIR}${file}")
    endif()
  else(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    message(STATUS "File $ENV{DESTDIR}${file} does not exist.")
  endif()
endforeach()
