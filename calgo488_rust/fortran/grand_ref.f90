! Independent modern Fortran reference for ACM CALGO 488 GRAND.
! Richard P. Brent, Communications of the ACM 17(12), 1974, pp. 704-706.
! Uses the same portable 32-bit LCG as the Rust program for repeatability.
module grand488
  use, intrinsic :: iso_fortran_env, only : real32, int64
  implicit none
  private
  public :: init, grand
  real(real32), parameter :: d(60) = [ &
     0.674489750, 0.475589630, 0.383771164, 0.328611323, &
     0.291142827, 0.263684322, 0.242508452, 0.225567444, &
     0.216346166, 0.199924267, 0.189917580, 0.181925181, &
     0.173601400, 0.166841909, 0.160796729, 0.155349717, &
     0.150409384, 0.145902577, 0.141770033, 0.137963174, &
     0.134441762, 0.131172150, 0.128259650, 0.125279090, &
     0.122610883, 0.120350610, 0.117741707, 0.115511892, &
     0.113402349, 0.111402720, 0.109503852, 0.107697617, &
     0.105976772, 0.104334841, 0.102766212, 0.101265052, &
     0.099827234, 0.098448282, 0.097124309, 0.095851178, &
     0.094627461, 0.093444847, 0.092311989, 0.091215482, &
     0.090156838, 0.089133867, 0.088144619, 0.087187293, &
     0.086260215, 0.085361834, 0.084490706, 0.083645487, &
     0.082824924, 0.082027847, 0.081253162, 0.080498544, &
     0.079769632, 0.078953527, 0.078358781, 0.077681899 ]
  integer(int64), save :: rng_state = 1_int64
  real(real32), save :: u = 0.0_real32
contains
  subroutine init(seed)
    integer(int64), intent(in) :: seed
    rng_state = seed
    u = 0.0_real32
  end subroutine

  real(real32) function uniform() result(r)
    rng_state = modulo(1664525_int64*rng_state + 1013904223_int64, 4294967296_int64)
    r = real(rng_state / 256_int64, real32) / 16777216.0_real32
  end function

  real(real32) function grand() result(x)
    integer :: i
    real(real32) :: a, w, v
    a = 0.0_real32
    i = 0
    do
      u = u + u
      if (u < 1.0_real32) exit
      u = u - 1.0_real32
      i = i + 1
      a = a - d(i)
    end do
    ! Label 20: rejection returns here without re-examining the saved bits.
    outer: do
      w = d(i+1) * u
      v = w * (0.5_real32*w-a)
      inner: do
        u = uniform()
        if (v <= u) exit inner
        v = uniform()
        if (u > v) cycle inner
        u = (v-u)/(1.0_real32-u)
        cycle outer
      end do inner
      u = (u-v)/(1.0_real32-v)
      u = u + u
      if (u < 1.0_real32) then
        x = a-w
      else
        u = u - 1.0_real32
        x = w-a
      end if
      return
    end do outer
  end function
end module grand488

program calgo488_fortran
  use, intrinsic :: iso_fortran_env, only : real32, int64
  use grand488
  implicit none
  character(64) :: arg, buf
  integer(int64) :: seed
  integer :: n, i, ios
  if (command_argument_count() /= 2) then
    write(*, '(A)') 'Usage: calgo488_fortran <unsigned-seed-0..4294967295> <count-1..1000000>'
    stop 2
  end if
  call get_command_argument(1, arg)
  read(arg, *, iostat=ios) seed
  if (ios /= 0) stop 2
  if (seed < 0_int64 .or. seed > 4294967295_int64) stop 2
  call get_command_argument(2, arg)
  read(arg, *, iostat=ios) n
  if (ios /= 0) stop 2
  if (n < 1 .or. n > 1000000) stop 2
  call init(seed)
  do i=1,n
    write(buf, '(F20.7)') grand()
    write(*, '(A)') trim(adjustl(buf))
  end do
end program calgo488_fortran
